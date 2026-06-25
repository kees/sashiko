// Copyright 2026 The Sashiko Authors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! A pass-through [`AiProvider`] decorator that caps the number of concurrent
//! `generate_content` calls.
//!
//! A review worker runs its analysis stages concurrently (`try_join_all`), so a
//! single review can fire one model call per stage at once. In daemon mode that
//! burst is throttled globally by the daemon's `llm_semaphore`, but a local
//! `--force-local` review has no daemon in front of it. Wrapping the worker's
//! provider with this decorator (in `bin/review.rs`, local mode only) makes a
//! local review honour `[review] concurrency` instead of firing every stage at
//! once.

use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use tokio::sync::Semaphore;

use crate::ai::{AiProvider, AiRequest, AiResponse, CacheStats, ProviderCapabilities};

/// Limits concurrent model calls to a fixed number of permits. All other
/// behaviour is delegated unchanged to the inner provider.
pub struct ConcurrencyLimitedProvider {
    inner: Arc<dyn AiProvider>,
    semaphore: Arc<Semaphore>,
}

impl ConcurrencyLimitedProvider {
    /// `limit` is clamped to at least 1 so the provider can never deadlock.
    pub fn new(inner: Arc<dyn AiProvider>, limit: usize) -> Self {
        Self {
            inner,
            semaphore: Arc::new(Semaphore::new(limit.max(1))),
        }
    }
}

#[async_trait]
impl AiProvider for ConcurrencyLimitedProvider {
    async fn generate_content(&self, request: AiRequest) -> Result<AiResponse> {
        let _permit = self
            .semaphore
            .acquire()
            .await
            .map_err(|e| anyhow::anyhow!("concurrency semaphore closed: {e}"))?;
        self.inner.generate_content(request).await
    }

    fn estimate_tokens(&self, request: &AiRequest) -> usize {
        self.inner.estimate_tokens(request)
    }

    fn get_capabilities(&self) -> ProviderCapabilities {
        self.inner.get_capabilities()
    }

    fn cache_stats(&self) -> Option<CacheStats> {
        self.inner.cache_stats()
    }
}
