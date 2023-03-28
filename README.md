# substrate-arkworks-examples
Example implementation of curve arithmetic for the ark-substrate curves.

## Benchmark results

| extrinsic                               |  normal(µs)      |optimized(µs)    |   speedup      | 
| --------------- |  --------------- | --------------- | --------------- | 
| groth16_verification (bls12_381)        |    23551.78      |    3548.19      |${\color{green}\bf 6.64 \boldsymbol{\times}}$| 
| groth16_prepare_inputs                   |    1796.18      |    266.89       |${\color{green}\bf 6.73 \boldsymbol{\times}}$|
| groth16_prepare_verifying_key            |    10917.28     |    1353.79      |${\color{green}\bf 8.06 \boldsymbol{\times}}$|
| groth16_verify_with_prepared_inputs      |    11761.42     |    1988.77      |${\color{green}\bf 5.91 \boldsymbol{\times}}$|

## Benchmarking

You can run the included benchmarks of the Substrate extrinsics on your local machine with:

```shell
gh repo clone https://github.com/achimcc/substrate-arkworks-examples
cd substrate-arkworks-examples
make install 
make benchmark
```

The results are then written into a benchmark.pdf file.

An overview and comparison of all benchmark results can be found [here](https://github.com/achimcc/substrate-arkworks-examples/blob/main/benchmarks-comparison.md).
