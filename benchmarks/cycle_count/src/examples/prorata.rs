// Copyright 2023 RISC Zero, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use prorata_core::AllocationQuery;
use risc0_zkvm::{serde::to_vec, ExecutorEnv};
use rust_decimal::Decimal;

use crate::{exec_compute, CycleCounter};

pub struct Job<'a> {
    pub env: ExecutorEnv<'a>,
}

const METHOD_ELF: &'static [u8] = prorata_methods::PRORATA_GUEST_ELF;

impl CycleCounter for Job<'_> {
    const NAME: &'static str = "prorata";

    fn new() -> Self {
        let recipients_csv = std::fs::read("../../examples/prorata/sample/ingen.csv")
            .expect("Failed to read input file");
        let target = "John Hammond".to_string();
        let amount = Decimal::from_str_radix("1000000000", 10).unwrap();
        let env = ExecutorEnv::builder()
            .add_input(
                &to_vec(&AllocationQuery {
                    amount,
                    recipients_csv,
                    target,
                })
                .unwrap(),
            )
            .build()
            .unwrap();

        Job { env }
    }

    fn exec_compute(&mut self) -> u32 {
        exec_compute(METHOD_ELF, self.env.clone())
    }
}
