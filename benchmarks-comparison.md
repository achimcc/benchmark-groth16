# Bechmark results comparison table


| extrinsic                               |  arkworks(µs)      |ark-substrate(µs)    |   speedup      | 
| --------------- |  --------------- | --------------- | --------------- | 
| groth16_verification (bls12_381)        |    23551.78      |    3548.19      |${\color{green}\bf 6.64 \boldsymbol{\times}}$| 
| groth16_prepare_inputs                   |    1796.18      |    266.89       |${\color{green}\bf 6.73 \boldsymbol{\times}}$|
| groth16_prepare_verifying_key            |    10917.28     |    1353.79      |${\color{green}\bf 8.06 \boldsymbol{\times}}$|
| groth16_verify_with_prepared_inputs      |    11761.42     |    1988.77      |${\color{green}\bf 5.91 \boldsymbol{\times}}$|
