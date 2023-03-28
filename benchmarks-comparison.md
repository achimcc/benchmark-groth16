# Bechmark results comparison table


| extrinsic                               |  normal(µs)[^1]  |optimized(µs)[^2]|   speedup[^3]   |  dummy(µs)[^4]  |   wasm(µs)[^5]  |  native(µs)[^6] |
| --------------------------------------- |  --------------- | --------------- | --------------- | --------------- | --------------- | --------------- |
| groth16_verification (bls12_381)        |    27519.38      |    7853.77      |${\color{green}\bf 3.50 \boldsymbol{\times}}$|                 |                     |                 | 
| groth16_prepare_inputs                   |    1847.27      |    274.39       |${\color{green}\bf 6.73 \boldsymbol{\times}}$|                 |                     |             |
| groth16_prepare_verifying_key            |    13873.53     |    4287.22      |${\color{green}\bf 3.24 \boldsymbol{\times}}$|                 |                     |             |
| groth16_verify_with_prepared_inputs      |    13226.42     |    3296.78      |${\color{green}\bf 4.01 \boldsymbol{\times}}$|                 |                     |             |
