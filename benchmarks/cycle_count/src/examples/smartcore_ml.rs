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

use risc0_zkvm::{serde::to_vec, ExecutorEnv};
use smartcore::{
    linalg::basic::matrix::DenseMatrix, tree::decision_tree_classifier::DecisionTreeClassifier,
};

use crate::{exec_compute, CycleCounter};

pub struct Job<'a> {
    pub env: ExecutorEnv<'a>,
}

const METHOD_ELF: &'static [u8] = smartcore_ml_methods::ML_TEMPLATE_ELF;

const JSON_MODEL: &str =
    include_str!("../../../../examples/smartcore-ml/res/ml-model/tree_model_bytes.json");
const JSON_DATA: &str =
    include_str!("../../../../examples/smartcore-ml/res/input-data/tree_model_data_bytes.json");

impl CycleCounter for Job<'_> {
    const NAME: &'static str = "smartcore-ml";

    fn new() -> Self {
        // Convert the model and input data from JSON into byte arrays.

        let is_svm = false;
        let model_bytes: Vec<u8> = serde_json::from_str(JSON_MODEL).unwrap();
        let data_bytes: Vec<u8> = serde_json::from_str(JSON_DATA).unwrap();

        // Deserialize the data from rmp into native rust types.
        type Model = DecisionTreeClassifier<f64, u32, DenseMatrix<f64>, Vec<u32>>;
        let model: Model =
            rmp_serde::from_slice(&model_bytes).expect("model failed to deserialize byte array");
        let data: DenseMatrix<f64> =
            rmp_serde::from_slice(&data_bytes).expect("data filed to deserialize byte array");

        let env = ExecutorEnv::builder()
            .add_input(&to_vec(&is_svm).expect("bool failed to serialize"))
            .add_input(&to_vec(&model).expect("model failed to serialize"))
            .add_input(&to_vec(&data).expect("data failed to serialize"))
            .build()
            .unwrap();

        Job { env }
    }

    fn exec_compute(&mut self) -> u32 {
        exec_compute(METHOD_ELF, self.env.clone())
    }
}
