
# Benchmark

Compares random element replacement.

For arrays, we check `Arrays.replace/3` as well as Access' `put_in`.
These are similar but slightly different APIs for element replacement.

For lists, `List.replace_at(list, index)` is used.


## System

Benchmark suite executing on the following system:

<table style="width: 1%">
  <tr>
    <th style="width: 1%; white-space: nowrap">Operating System</th>
    <td>Linux</td>
  </tr><tr>
    <th style="white-space: nowrap">CPU Information</th>
    <td style="white-space: nowrap">Intel(R) Core(TM) i7-6700HQ CPU @ 2.60GHz</td>
  </tr><tr>
    <th style="white-space: nowrap">Number of Available Cores</th>
    <td style="white-space: nowrap">8</td>
  </tr><tr>
    <th style="white-space: nowrap">Available Memory</th>
    <td style="white-space: nowrap">7.60 GB</td>
  </tr><tr>
    <th style="white-space: nowrap">Elixir Version</th>
    <td style="white-space: nowrap">1.12.2</td>
  </tr><tr>
    <th style="white-space: nowrap">Erlang Version</th>
    <td style="white-space: nowrap">24.0.1</td>
  </tr>
</table>

## Configuration

Benchmark suite executing with the following configuration:

<table style="width: 1%">
  <tr>
    <th style="width: 1%">:time</th>
    <td style="white-space: nowrap">500 ms</td>
  </tr><tr>
    <th>:parallel</th>
    <td style="white-space: nowrap">1</td>
  </tr><tr>
    <th>:warmup</th>
    <td style="white-space: nowrap">500 ms</td>
  </tr>
</table>

## Statistics




__Input: 0000000032 elements__

Run Time

<table style="width: 1%">
  <tr>
    <th>Name</th>
    <th style="text-align: right">IPS</th>
    <th style="text-align: right">Average</th>
    <th style="text-align: right">Devitation</th>
    <th style="text-align: right">Median</th>
    <th style="text-align: right">99th&nbsp;%</th>
  </tr>

  <tr>
    <td style="white-space: nowrap">List.replace_at/3</td>
    <td style="white-space: nowrap; text-align: right">3.56 M</td>
    <td style="white-space: nowrap; text-align: right">280.91 ns</td>
    <td style="white-space: nowrap; text-align: right">±593.92%</td>
    <td style="white-space: nowrap; text-align: right">192 ns</td>
    <td style="white-space: nowrap; text-align: right">538 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">2.15 M</td>
    <td style="white-space: nowrap; text-align: right">465.45 ns</td>
    <td style="white-space: nowrap; text-align: right">±655.26%</td>
    <td style="white-space: nowrap; text-align: right">223 ns</td>
    <td style="white-space: nowrap; text-align: right">679.40 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">2.00 M</td>
    <td style="white-space: nowrap; text-align: right">500.37 ns</td>
    <td style="white-space: nowrap; text-align: right">±661.88%</td>
    <td style="white-space: nowrap; text-align: right">247 ns</td>
    <td style="white-space: nowrap; text-align: right">680 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.68 M</td>
    <td style="white-space: nowrap; text-align: right">1473.33 ns</td>
    <td style="white-space: nowrap; text-align: right">±199.40%</td>
    <td style="white-space: nowrap; text-align: right">1251 ns</td>
    <td style="white-space: nowrap; text-align: right">3704.05 ns</td>
  </tr>

</table>


Comparison

<table style="width: 1%">
  <tr>
    <th>Name</th>
    <th style="text-align: right">IPS</th>
    <th style="text-align: right">Slower</th>
  <tr>
    <td style="white-space: nowrap">List.replace_at/3</td>
    <td style="white-space: nowrap;text-align: right">3.56 M</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">2.15 M</td>
    <td style="white-space: nowrap; text-align: right">1.66x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">2.00 M</td>
    <td style="white-space: nowrap; text-align: right">1.78x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.68 M</td>
    <td style="white-space: nowrap; text-align: right">5.24x</td>
  </tr>

</table>



<hr/>


__Input: 0000000064 elements__

Run Time

<table style="width: 1%">
  <tr>
    <th>Name</th>
    <th style="text-align: right">IPS</th>
    <th style="text-align: right">Average</th>
    <th style="text-align: right">Devitation</th>
    <th style="text-align: right">Median</th>
    <th style="text-align: right">99th&nbsp;%</th>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">3.12 M</td>
    <td style="white-space: nowrap; text-align: right">320.91 ns</td>
    <td style="white-space: nowrap; text-align: right">±613.63%</td>
    <td style="white-space: nowrap; text-align: right">229 ns</td>
    <td style="white-space: nowrap; text-align: right">425.93 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">3.07 M</td>
    <td style="white-space: nowrap; text-align: right">325.70 ns</td>
    <td style="white-space: nowrap; text-align: right">±361.57%</td>
    <td style="white-space: nowrap; text-align: right">251 ns</td>
    <td style="white-space: nowrap; text-align: right">786 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">List.replace_at/3</td>
    <td style="white-space: nowrap; text-align: right">2.33 M</td>
    <td style="white-space: nowrap; text-align: right">428.73 ns</td>
    <td style="white-space: nowrap; text-align: right">±528.73%</td>
    <td style="white-space: nowrap; text-align: right">292 ns</td>
    <td style="white-space: nowrap; text-align: right">759.55 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.60 M</td>
    <td style="white-space: nowrap; text-align: right">1672.39 ns</td>
    <td style="white-space: nowrap; text-align: right">±184.15%</td>
    <td style="white-space: nowrap; text-align: right">1430 ns</td>
    <td style="white-space: nowrap; text-align: right">7892.65 ns</td>
  </tr>

</table>


Comparison

<table style="width: 1%">
  <tr>
    <th>Name</th>
    <th style="text-align: right">IPS</th>
    <th style="text-align: right">Slower</th>
  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (ErlangArray)</td>
    <td style="white-space: nowrap;text-align: right">3.12 M</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">3.07 M</td>
    <td style="white-space: nowrap; text-align: right">1.01x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">List.replace_at/3</td>
    <td style="white-space: nowrap; text-align: right">2.33 M</td>
    <td style="white-space: nowrap; text-align: right">1.34x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.60 M</td>
    <td style="white-space: nowrap; text-align: right">5.21x</td>
  </tr>

</table>



<hr/>


__Input: 0000000128 elements__

Run Time

<table style="width: 1%">
  <tr>
    <th>Name</th>
    <th style="text-align: right">IPS</th>
    <th style="text-align: right">Average</th>
    <th style="text-align: right">Devitation</th>
    <th style="text-align: right">Median</th>
    <th style="text-align: right">99th&nbsp;%</th>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">3.05 M</td>
    <td style="white-space: nowrap; text-align: right">328.14 ns</td>
    <td style="white-space: nowrap; text-align: right">±530.56%</td>
    <td style="white-space: nowrap; text-align: right">244 ns</td>
    <td style="white-space: nowrap; text-align: right">480 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">3.05 M</td>
    <td style="white-space: nowrap; text-align: right">328.29 ns</td>
    <td style="white-space: nowrap; text-align: right">±417.12%</td>
    <td style="white-space: nowrap; text-align: right">260 ns</td>
    <td style="white-space: nowrap; text-align: right">502.22 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">List.replace_at/3</td>
    <td style="white-space: nowrap; text-align: right">1.47 M</td>
    <td style="white-space: nowrap; text-align: right">682.20 ns</td>
    <td style="white-space: nowrap; text-align: right">±383.63%</td>
    <td style="white-space: nowrap; text-align: right">462 ns</td>
    <td style="white-space: nowrap; text-align: right">4639.90 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.58 M</td>
    <td style="white-space: nowrap; text-align: right">1710.59 ns</td>
    <td style="white-space: nowrap; text-align: right">±160.88%</td>
    <td style="white-space: nowrap; text-align: right">1485 ns</td>
    <td style="white-space: nowrap; text-align: right">5479.78 ns</td>
  </tr>

</table>


Comparison

<table style="width: 1%">
  <tr>
    <th>Name</th>
    <th style="text-align: right">IPS</th>
    <th style="text-align: right">Slower</th>
  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (ErlangArray)</td>
    <td style="white-space: nowrap;text-align: right">3.05 M</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">3.05 M</td>
    <td style="white-space: nowrap; text-align: right">1.0x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">List.replace_at/3</td>
    <td style="white-space: nowrap; text-align: right">1.47 M</td>
    <td style="white-space: nowrap; text-align: right">2.08x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.58 M</td>
    <td style="white-space: nowrap; text-align: right">5.21x</td>
  </tr>

</table>



<hr/>


__Input: 0000000256 elements__

Run Time

<table style="width: 1%">
  <tr>
    <th>Name</th>
    <th style="text-align: right">IPS</th>
    <th style="text-align: right">Average</th>
    <th style="text-align: right">Devitation</th>
    <th style="text-align: right">Median</th>
    <th style="text-align: right">99th&nbsp;%</th>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">3.15 M</td>
    <td style="white-space: nowrap; text-align: right">0.32 μs</td>
    <td style="white-space: nowrap; text-align: right">±335.67%</td>
    <td style="white-space: nowrap; text-align: right">0.26 μs</td>
    <td style="white-space: nowrap; text-align: right">0.52 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">2.97 M</td>
    <td style="white-space: nowrap; text-align: right">0.34 μs</td>
    <td style="white-space: nowrap; text-align: right">±407.47%</td>
    <td style="white-space: nowrap; text-align: right">0.28 μs</td>
    <td style="white-space: nowrap; text-align: right">0.51 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">List.replace_at/3</td>
    <td style="white-space: nowrap; text-align: right">0.85 M</td>
    <td style="white-space: nowrap; text-align: right">1.18 μs</td>
    <td style="white-space: nowrap; text-align: right">±303.23%</td>
    <td style="white-space: nowrap; text-align: right">0.81 μs</td>
    <td style="white-space: nowrap; text-align: right">14.08 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.57 M</td>
    <td style="white-space: nowrap; text-align: right">1.77 μs</td>
    <td style="white-space: nowrap; text-align: right">±142.28%</td>
    <td style="white-space: nowrap; text-align: right">1.51 μs</td>
    <td style="white-space: nowrap; text-align: right">6.35 μs</td>
  </tr>

</table>


Comparison

<table style="width: 1%">
  <tr>
    <th>Name</th>
    <th style="text-align: right">IPS</th>
    <th style="text-align: right">Slower</th>
  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (ErlangArray)</td>
    <td style="white-space: nowrap;text-align: right">3.15 M</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">2.97 M</td>
    <td style="white-space: nowrap; text-align: right">1.06x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">List.replace_at/3</td>
    <td style="white-space: nowrap; text-align: right">0.85 M</td>
    <td style="white-space: nowrap; text-align: right">3.71x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.57 M</td>
    <td style="white-space: nowrap; text-align: right">5.56x</td>
  </tr>

</table>



<hr/>


__Input: 0000000512 elements__

Run Time

<table style="width: 1%">
  <tr>
    <th>Name</th>
    <th style="text-align: right">IPS</th>
    <th style="text-align: right">Average</th>
    <th style="text-align: right">Devitation</th>
    <th style="text-align: right">Median</th>
    <th style="text-align: right">99th&nbsp;%</th>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">2.59 M</td>
    <td style="white-space: nowrap; text-align: right">0.39 μs</td>
    <td style="white-space: nowrap; text-align: right">±526.01%</td>
    <td style="white-space: nowrap; text-align: right">0.28 μs</td>
    <td style="white-space: nowrap; text-align: right">0.78 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">2.43 M</td>
    <td style="white-space: nowrap; text-align: right">0.41 μs</td>
    <td style="white-space: nowrap; text-align: right">±422.17%</td>
    <td style="white-space: nowrap; text-align: right">0.30 μs</td>
    <td style="white-space: nowrap; text-align: right">1.68 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.55 M</td>
    <td style="white-space: nowrap; text-align: right">1.81 μs</td>
    <td style="white-space: nowrap; text-align: right">±273.09%</td>
    <td style="white-space: nowrap; text-align: right">1.55 μs</td>
    <td style="white-space: nowrap; text-align: right">5.91 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">List.replace_at/3</td>
    <td style="white-space: nowrap; text-align: right">0.44 M</td>
    <td style="white-space: nowrap; text-align: right">2.25 μs</td>
    <td style="white-space: nowrap; text-align: right">±230.12%</td>
    <td style="white-space: nowrap; text-align: right">1.53 μs</td>
    <td style="white-space: nowrap; text-align: right">25.26 μs</td>
  </tr>

</table>


Comparison

<table style="width: 1%">
  <tr>
    <th>Name</th>
    <th style="text-align: right">IPS</th>
    <th style="text-align: right">Slower</th>
  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (ErlangArray)</td>
    <td style="white-space: nowrap;text-align: right">2.59 M</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">2.43 M</td>
    <td style="white-space: nowrap; text-align: right">1.07x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.55 M</td>
    <td style="white-space: nowrap; text-align: right">4.68x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">List.replace_at/3</td>
    <td style="white-space: nowrap; text-align: right">0.44 M</td>
    <td style="white-space: nowrap; text-align: right">5.82x</td>
  </tr>

</table>



<hr/>


__Input: 0000001024 elements__

Run Time

<table style="width: 1%">
  <tr>
    <th>Name</th>
    <th style="text-align: right">IPS</th>
    <th style="text-align: right">Average</th>
    <th style="text-align: right">Devitation</th>
    <th style="text-align: right">Median</th>
    <th style="text-align: right">99th&nbsp;%</th>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">2.62 M</td>
    <td style="white-space: nowrap; text-align: right">0.38 μs</td>
    <td style="white-space: nowrap; text-align: right">±535.72%</td>
    <td style="white-space: nowrap; text-align: right">0.28 μs</td>
    <td style="white-space: nowrap; text-align: right">0.65 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">2.50 M</td>
    <td style="white-space: nowrap; text-align: right">0.40 μs</td>
    <td style="white-space: nowrap; text-align: right">±643.40%</td>
    <td style="white-space: nowrap; text-align: right">0.30 μs</td>
    <td style="white-space: nowrap; text-align: right">0.65 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.53 M</td>
    <td style="white-space: nowrap; text-align: right">1.88 μs</td>
    <td style="white-space: nowrap; text-align: right">±160.74%</td>
    <td style="white-space: nowrap; text-align: right">1.63 μs</td>
    <td style="white-space: nowrap; text-align: right">6.07 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">List.replace_at/3</td>
    <td style="white-space: nowrap; text-align: right">0.22 M</td>
    <td style="white-space: nowrap; text-align: right">4.56 μs</td>
    <td style="white-space: nowrap; text-align: right">±176.17%</td>
    <td style="white-space: nowrap; text-align: right">3.14 μs</td>
    <td style="white-space: nowrap; text-align: right">47.84 μs</td>
  </tr>

</table>


Comparison

<table style="width: 1%">
  <tr>
    <th>Name</th>
    <th style="text-align: right">IPS</th>
    <th style="text-align: right">Slower</th>
  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (ErlangArray)</td>
    <td style="white-space: nowrap;text-align: right">2.62 M</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">2.50 M</td>
    <td style="white-space: nowrap; text-align: right">1.05x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.53 M</td>
    <td style="white-space: nowrap; text-align: right">4.93x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">List.replace_at/3</td>
    <td style="white-space: nowrap; text-align: right">0.22 M</td>
    <td style="white-space: nowrap; text-align: right">11.96x</td>
  </tr>

</table>



<hr/>


__Input: 0000002048 elements__

Run Time

<table style="width: 1%">
  <tr>
    <th>Name</th>
    <th style="text-align: right">IPS</th>
    <th style="text-align: right">Average</th>
    <th style="text-align: right">Devitation</th>
    <th style="text-align: right">Median</th>
    <th style="text-align: right">99th&nbsp;%</th>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">2.54 M</td>
    <td style="white-space: nowrap; text-align: right">0.39 μs</td>
    <td style="white-space: nowrap; text-align: right">±387.01%</td>
    <td style="white-space: nowrap; text-align: right">0.31 μs</td>
    <td style="white-space: nowrap; text-align: right">0.78 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">2.42 M</td>
    <td style="white-space: nowrap; text-align: right">0.41 μs</td>
    <td style="white-space: nowrap; text-align: right">±418.66%</td>
    <td style="white-space: nowrap; text-align: right">0.32 μs</td>
    <td style="white-space: nowrap; text-align: right">0.72 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.49 M</td>
    <td style="white-space: nowrap; text-align: right">2.06 μs</td>
    <td style="white-space: nowrap; text-align: right">±133.07%</td>
    <td style="white-space: nowrap; text-align: right">1.80 μs</td>
    <td style="white-space: nowrap; text-align: right">15.03 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">List.replace_at/3</td>
    <td style="white-space: nowrap; text-align: right">0.112 M</td>
    <td style="white-space: nowrap; text-align: right">8.94 μs</td>
    <td style="white-space: nowrap; text-align: right">±132.90%</td>
    <td style="white-space: nowrap; text-align: right">6.22 μs</td>
    <td style="white-space: nowrap; text-align: right">65.53 μs</td>
  </tr>

</table>


Comparison

<table style="width: 1%">
  <tr>
    <th>Name</th>
    <th style="text-align: right">IPS</th>
    <th style="text-align: right">Slower</th>
  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (ErlangArray)</td>
    <td style="white-space: nowrap;text-align: right">2.54 M</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">2.42 M</td>
    <td style="white-space: nowrap; text-align: right">1.05x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.49 M</td>
    <td style="white-space: nowrap; text-align: right">5.21x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">List.replace_at/3</td>
    <td style="white-space: nowrap; text-align: right">0.112 M</td>
    <td style="white-space: nowrap; text-align: right">22.66x</td>
  </tr>

</table>



<hr/>


__Input: 0000004096 elements__

Run Time

<table style="width: 1%">
  <tr>
    <th>Name</th>
    <th style="text-align: right">IPS</th>
    <th style="text-align: right">Average</th>
    <th style="text-align: right">Devitation</th>
    <th style="text-align: right">Median</th>
    <th style="text-align: right">99th&nbsp;%</th>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">2.22 M</td>
    <td style="white-space: nowrap; text-align: right">0.45 μs</td>
    <td style="white-space: nowrap; text-align: right">±427.85%</td>
    <td style="white-space: nowrap; text-align: right">0.35 μs</td>
    <td style="white-space: nowrap; text-align: right">0.75 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">2.16 M</td>
    <td style="white-space: nowrap; text-align: right">0.46 μs</td>
    <td style="white-space: nowrap; text-align: right">±544.02%</td>
    <td style="white-space: nowrap; text-align: right">0.34 μs</td>
    <td style="white-space: nowrap; text-align: right">0.77 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.41 M</td>
    <td style="white-space: nowrap; text-align: right">2.45 μs</td>
    <td style="white-space: nowrap; text-align: right">±118.06%</td>
    <td style="white-space: nowrap; text-align: right">2.17 μs</td>
    <td style="white-space: nowrap; text-align: right">17.92 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">List.replace_at/3</td>
    <td style="white-space: nowrap; text-align: right">0.0568 M</td>
    <td style="white-space: nowrap; text-align: right">17.61 μs</td>
    <td style="white-space: nowrap; text-align: right">±107.94%</td>
    <td style="white-space: nowrap; text-align: right">12.57 μs</td>
    <td style="white-space: nowrap; text-align: right">80.91 μs</td>
  </tr>

</table>


Comparison

<table style="width: 1%">
  <tr>
    <th>Name</th>
    <th style="text-align: right">IPS</th>
    <th style="text-align: right">Slower</th>
  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (MapArray)</td>
    <td style="white-space: nowrap;text-align: right">2.22 M</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">2.16 M</td>
    <td style="white-space: nowrap; text-align: right">1.03x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.41 M</td>
    <td style="white-space: nowrap; text-align: right">5.43x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">List.replace_at/3</td>
    <td style="white-space: nowrap; text-align: right">0.0568 M</td>
    <td style="white-space: nowrap; text-align: right">39.1x</td>
  </tr>

</table>



<hr/>


__Input: 0000008192 elements__

Run Time

<table style="width: 1%">
  <tr>
    <th>Name</th>
    <th style="text-align: right">IPS</th>
    <th style="text-align: right">Average</th>
    <th style="text-align: right">Devitation</th>
    <th style="text-align: right">Median</th>
    <th style="text-align: right">99th&nbsp;%</th>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">2.14 M</td>
    <td style="white-space: nowrap; text-align: right">0.47 μs</td>
    <td style="white-space: nowrap; text-align: right">±183.47%</td>
    <td style="white-space: nowrap; text-align: right">0.39 μs</td>
    <td style="white-space: nowrap; text-align: right">0.89 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">2.10 M</td>
    <td style="white-space: nowrap; text-align: right">0.48 μs</td>
    <td style="white-space: nowrap; text-align: right">±226.89%</td>
    <td style="white-space: nowrap; text-align: right">0.38 μs</td>
    <td style="white-space: nowrap; text-align: right">0.82 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.38 M</td>
    <td style="white-space: nowrap; text-align: right">2.66 μs</td>
    <td style="white-space: nowrap; text-align: right">±105.08%</td>
    <td style="white-space: nowrap; text-align: right">2.37 μs</td>
    <td style="white-space: nowrap; text-align: right">15.23 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">List.replace_at/3</td>
    <td style="white-space: nowrap; text-align: right">0.0289 M</td>
    <td style="white-space: nowrap; text-align: right">34.60 μs</td>
    <td style="white-space: nowrap; text-align: right">±106.02%</td>
    <td style="white-space: nowrap; text-align: right">25.95 μs</td>
    <td style="white-space: nowrap; text-align: right">122.36 μs</td>
  </tr>

</table>


Comparison

<table style="width: 1%">
  <tr>
    <th>Name</th>
    <th style="text-align: right">IPS</th>
    <th style="text-align: right">Slower</th>
  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (MapArray)</td>
    <td style="white-space: nowrap;text-align: right">2.14 M</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">2.10 M</td>
    <td style="white-space: nowrap; text-align: right">1.02x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.38 M</td>
    <td style="white-space: nowrap; text-align: right">5.68x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">List.replace_at/3</td>
    <td style="white-space: nowrap; text-align: right">0.0289 M</td>
    <td style="white-space: nowrap; text-align: right">73.9x</td>
  </tr>

</table>



<hr/>


__Input: 0000016384 elements__

Run Time

<table style="width: 1%">
  <tr>
    <th>Name</th>
    <th style="text-align: right">IPS</th>
    <th style="text-align: right">Average</th>
    <th style="text-align: right">Devitation</th>
    <th style="text-align: right">Median</th>
    <th style="text-align: right">99th&nbsp;%</th>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">2.05 M</td>
    <td style="white-space: nowrap; text-align: right">0.49 μs</td>
    <td style="white-space: nowrap; text-align: right">±137.69%</td>
    <td style="white-space: nowrap; text-align: right">0.40 μs</td>
    <td style="white-space: nowrap; text-align: right">1.00 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">1.70 M</td>
    <td style="white-space: nowrap; text-align: right">0.59 μs</td>
    <td style="white-space: nowrap; text-align: right">±175.35%</td>
    <td style="white-space: nowrap; text-align: right">0.41 μs</td>
    <td style="white-space: nowrap; text-align: right">1.84 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.35 M</td>
    <td style="white-space: nowrap; text-align: right">2.85 μs</td>
    <td style="white-space: nowrap; text-align: right">±114.58%</td>
    <td style="white-space: nowrap; text-align: right">2.51 μs</td>
    <td style="white-space: nowrap; text-align: right">18.82 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">List.replace_at/3</td>
    <td style="white-space: nowrap; text-align: right">0.0144 M</td>
    <td style="white-space: nowrap; text-align: right">69.28 μs</td>
    <td style="white-space: nowrap; text-align: right">±81.09%</td>
    <td style="white-space: nowrap; text-align: right">59.18 μs</td>
    <td style="white-space: nowrap; text-align: right">246.12 μs</td>
  </tr>

</table>


Comparison

<table style="width: 1%">
  <tr>
    <th>Name</th>
    <th style="text-align: right">IPS</th>
    <th style="text-align: right">Slower</th>
  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (MapArray)</td>
    <td style="white-space: nowrap;text-align: right">2.05 M</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">1.70 M</td>
    <td style="white-space: nowrap; text-align: right">1.2x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.35 M</td>
    <td style="white-space: nowrap; text-align: right">5.83x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">List.replace_at/3</td>
    <td style="white-space: nowrap; text-align: right">0.0144 M</td>
    <td style="white-space: nowrap; text-align: right">141.83x</td>
  </tr>

</table>



<hr/>


__Input: 0000032768 elements__

Run Time

<table style="width: 1%">
  <tr>
    <th>Name</th>
    <th style="text-align: right">IPS</th>
    <th style="text-align: right">Average</th>
    <th style="text-align: right">Devitation</th>
    <th style="text-align: right">Median</th>
    <th style="text-align: right">99th&nbsp;%</th>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">543.78 K</td>
    <td style="white-space: nowrap; text-align: right">1.84 μs</td>
    <td style="white-space: nowrap; text-align: right">±1423.20%</td>
    <td style="white-space: nowrap; text-align: right">0.46 μs</td>
    <td style="white-space: nowrap; text-align: right">2.49 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">447.06 K</td>
    <td style="white-space: nowrap; text-align: right">2.24 μs</td>
    <td style="white-space: nowrap; text-align: right">±1597.08%</td>
    <td style="white-space: nowrap; text-align: right">0.47 μs</td>
    <td style="white-space: nowrap; text-align: right">1.76 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">337.73 K</td>
    <td style="white-space: nowrap; text-align: right">2.96 μs</td>
    <td style="white-space: nowrap; text-align: right">±125.46%</td>
    <td style="white-space: nowrap; text-align: right">2.53 μs</td>
    <td style="white-space: nowrap; text-align: right">18.99 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">List.replace_at/3</td>
    <td style="white-space: nowrap; text-align: right">5.44 K</td>
    <td style="white-space: nowrap; text-align: right">183.81 μs</td>
    <td style="white-space: nowrap; text-align: right">±95.60%</td>
    <td style="white-space: nowrap; text-align: right">125.38 μs</td>
    <td style="white-space: nowrap; text-align: right">765.31 μs</td>
  </tr>

</table>


Comparison

<table style="width: 1%">
  <tr>
    <th>Name</th>
    <th style="text-align: right">IPS</th>
    <th style="text-align: right">Slower</th>
  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (ErlangArray)</td>
    <td style="white-space: nowrap;text-align: right">543.78 K</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">447.06 K</td>
    <td style="white-space: nowrap; text-align: right">1.22x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">337.73 K</td>
    <td style="white-space: nowrap; text-align: right">1.61x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">List.replace_at/3</td>
    <td style="white-space: nowrap; text-align: right">5.44 K</td>
    <td style="white-space: nowrap; text-align: right">99.95x</td>
  </tr>

</table>



<hr/>


__Input: 0000065536 elements__

Run Time

<table style="width: 1%">
  <tr>
    <th>Name</th>
    <th style="text-align: right">IPS</th>
    <th style="text-align: right">Average</th>
    <th style="text-align: right">Devitation</th>
    <th style="text-align: right">Median</th>
    <th style="text-align: right">99th&nbsp;%</th>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">743.66 K</td>
    <td style="white-space: nowrap; text-align: right">1.34 μs</td>
    <td style="white-space: nowrap; text-align: right">±111.21%</td>
    <td style="white-space: nowrap; text-align: right">1.20 μs</td>
    <td style="white-space: nowrap; text-align: right">9.42 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">381.97 K</td>
    <td style="white-space: nowrap; text-align: right">2.62 μs</td>
    <td style="white-space: nowrap; text-align: right">±79.49%</td>
    <td style="white-space: nowrap; text-align: right">2.41 μs</td>
    <td style="white-space: nowrap; text-align: right">17.97 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">317.69 K</td>
    <td style="white-space: nowrap; text-align: right">3.15 μs</td>
    <td style="white-space: nowrap; text-align: right">±105.87%</td>
    <td style="white-space: nowrap; text-align: right">2.73 μs</td>
    <td style="white-space: nowrap; text-align: right">19.33 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">List.replace_at/3</td>
    <td style="white-space: nowrap; text-align: right">2.93 K</td>
    <td style="white-space: nowrap; text-align: right">340.77 μs</td>
    <td style="white-space: nowrap; text-align: right">±85.88%</td>
    <td style="white-space: nowrap; text-align: right">214.36 μs</td>
    <td style="white-space: nowrap; text-align: right">1085.54 μs</td>
  </tr>

</table>


Comparison

<table style="width: 1%">
  <tr>
    <th>Name</th>
    <th style="text-align: right">IPS</th>
    <th style="text-align: right">Slower</th>
  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (ErlangArray)</td>
    <td style="white-space: nowrap;text-align: right">743.66 K</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">381.97 K</td>
    <td style="white-space: nowrap; text-align: right">1.95x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">317.69 K</td>
    <td style="white-space: nowrap; text-align: right">2.34x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">List.replace_at/3</td>
    <td style="white-space: nowrap; text-align: right">2.93 K</td>
    <td style="white-space: nowrap; text-align: right">253.42x</td>
  </tr>

</table>



<hr/>


__Input: 0000131072 elements__

Run Time

<table style="width: 1%">
  <tr>
    <th>Name</th>
    <th style="text-align: right">IPS</th>
    <th style="text-align: right">Average</th>
    <th style="text-align: right">Devitation</th>
    <th style="text-align: right">Median</th>
    <th style="text-align: right">99th&nbsp;%</th>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">294.00 K</td>
    <td style="white-space: nowrap; text-align: right">3.40 μs</td>
    <td style="white-space: nowrap; text-align: right">±104.42%</td>
    <td style="white-space: nowrap; text-align: right">2.95 μs</td>
    <td style="white-space: nowrap; text-align: right">20.87 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">272.55 K</td>
    <td style="white-space: nowrap; text-align: right">3.67 μs</td>
    <td style="white-space: nowrap; text-align: right">±32.54%</td>
    <td style="white-space: nowrap; text-align: right">3.50 μs</td>
    <td style="white-space: nowrap; text-align: right">10.96 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">272.00 K</td>
    <td style="white-space: nowrap; text-align: right">3.68 μs</td>
    <td style="white-space: nowrap; text-align: right">±49.96%</td>
    <td style="white-space: nowrap; text-align: right">3.52 μs</td>
    <td style="white-space: nowrap; text-align: right">18.38 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">List.replace_at/3</td>
    <td style="white-space: nowrap; text-align: right">1.29 K</td>
    <td style="white-space: nowrap; text-align: right">776.02 μs</td>
    <td style="white-space: nowrap; text-align: right">±105.93%</td>
    <td style="white-space: nowrap; text-align: right">585.08 μs</td>
    <td style="white-space: nowrap; text-align: right">3931.74 μs</td>
  </tr>

</table>


Comparison

<table style="width: 1%">
  <tr>
    <th>Name</th>
    <th style="text-align: right">IPS</th>
    <th style="text-align: right">Slower</th>
  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (RRBVector)</td>
    <td style="white-space: nowrap;text-align: right">294.00 K</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">272.55 K</td>
    <td style="white-space: nowrap; text-align: right">1.08x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">272.00 K</td>
    <td style="white-space: nowrap; text-align: right">1.08x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">List.replace_at/3</td>
    <td style="white-space: nowrap; text-align: right">1.29 K</td>
    <td style="white-space: nowrap; text-align: right">228.15x</td>
  </tr>

</table>



<hr/>


__Input: 0000262144 elements__

Run Time

<table style="width: 1%">
  <tr>
    <th>Name</th>
    <th style="text-align: right">IPS</th>
    <th style="text-align: right">Average</th>
    <th style="text-align: right">Devitation</th>
    <th style="text-align: right">Median</th>
    <th style="text-align: right">99th&nbsp;%</th>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">254.97 K</td>
    <td style="white-space: nowrap; text-align: right">3.92 μs</td>
    <td style="white-space: nowrap; text-align: right">±74.63%</td>
    <td style="white-space: nowrap; text-align: right">3.45 μs</td>
    <td style="white-space: nowrap; text-align: right">20.86 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">251.47 K</td>
    <td style="white-space: nowrap; text-align: right">3.98 μs</td>
    <td style="white-space: nowrap; text-align: right">±49.53%</td>
    <td style="white-space: nowrap; text-align: right">3.67 μs</td>
    <td style="white-space: nowrap; text-align: right">17.17 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">233.64 K</td>
    <td style="white-space: nowrap; text-align: right">4.28 μs</td>
    <td style="white-space: nowrap; text-align: right">±77.51%</td>
    <td style="white-space: nowrap; text-align: right">3.68 μs</td>
    <td style="white-space: nowrap; text-align: right">28.51 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">List.replace_at/3</td>
    <td style="white-space: nowrap; text-align: right">0.51 K</td>
    <td style="white-space: nowrap; text-align: right">1965.47 μs</td>
    <td style="white-space: nowrap; text-align: right">±92.86%</td>
    <td style="white-space: nowrap; text-align: right">1157.58 μs</td>
    <td style="white-space: nowrap; text-align: right">5920.58 μs</td>
  </tr>

</table>


Comparison

<table style="width: 1%">
  <tr>
    <th>Name</th>
    <th style="text-align: right">IPS</th>
    <th style="text-align: right">Slower</th>
  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (RRBVector)</td>
    <td style="white-space: nowrap;text-align: right">254.97 K</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">251.47 K</td>
    <td style="white-space: nowrap; text-align: right">1.01x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">233.64 K</td>
    <td style="white-space: nowrap; text-align: right">1.09x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">List.replace_at/3</td>
    <td style="white-space: nowrap; text-align: right">0.51 K</td>
    <td style="white-space: nowrap; text-align: right">501.13x</td>
  </tr>

</table>



<hr/>


__Input: 0000524288 elements__

Run Time

<table style="width: 1%">
  <tr>
    <th>Name</th>
    <th style="text-align: right">IPS</th>
    <th style="text-align: right">Average</th>
    <th style="text-align: right">Devitation</th>
    <th style="text-align: right">Median</th>
    <th style="text-align: right">99th&nbsp;%</th>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">247.50 K</td>
    <td style="white-space: nowrap; text-align: right">4.04 μs</td>
    <td style="white-space: nowrap; text-align: right">±72.40%</td>
    <td style="white-space: nowrap; text-align: right">3.57 μs</td>
    <td style="white-space: nowrap; text-align: right">20.48 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">238.55 K</td>
    <td style="white-space: nowrap; text-align: right">4.19 μs</td>
    <td style="white-space: nowrap; text-align: right">±68.16%</td>
    <td style="white-space: nowrap; text-align: right">3.55 μs</td>
    <td style="white-space: nowrap; text-align: right">22.79 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">234.55 K</td>
    <td style="white-space: nowrap; text-align: right">4.26 μs</td>
    <td style="white-space: nowrap; text-align: right">±56.81%</td>
    <td style="white-space: nowrap; text-align: right">3.61 μs</td>
    <td style="white-space: nowrap; text-align: right">15.68 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">List.replace_at/3</td>
    <td style="white-space: nowrap; text-align: right">0.159 K</td>
    <td style="white-space: nowrap; text-align: right">6272.05 μs</td>
    <td style="white-space: nowrap; text-align: right">±95.95%</td>
    <td style="white-space: nowrap; text-align: right">5295.36 μs</td>
    <td style="white-space: nowrap; text-align: right">28118.32 μs</td>
  </tr>

</table>


Comparison

<table style="width: 1%">
  <tr>
    <th>Name</th>
    <th style="text-align: right">IPS</th>
    <th style="text-align: right">Slower</th>
  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (RRBVector)</td>
    <td style="white-space: nowrap;text-align: right">247.50 K</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">238.55 K</td>
    <td style="white-space: nowrap; text-align: right">1.04x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">234.55 K</td>
    <td style="white-space: nowrap; text-align: right">1.06x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">List.replace_at/3</td>
    <td style="white-space: nowrap; text-align: right">0.159 K</td>
    <td style="white-space: nowrap; text-align: right">1552.3x</td>
  </tr>

</table>



<hr/>


__Input: 0001048576 elements__

Run Time

<table style="width: 1%">
  <tr>
    <th>Name</th>
    <th style="text-align: right">IPS</th>
    <th style="text-align: right">Average</th>
    <th style="text-align: right">Devitation</th>
    <th style="text-align: right">Median</th>
    <th style="text-align: right">99th&nbsp;%</th>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">219.93 K</td>
    <td style="white-space: nowrap; text-align: right">4.55 μs</td>
    <td style="white-space: nowrap; text-align: right">±116.04%</td>
    <td style="white-space: nowrap; text-align: right">3.84 μs</td>
    <td style="white-space: nowrap; text-align: right">22.29 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">206.73 K</td>
    <td style="white-space: nowrap; text-align: right">4.84 μs</td>
    <td style="white-space: nowrap; text-align: right">±66.68%</td>
    <td style="white-space: nowrap; text-align: right">3.95 μs</td>
    <td style="white-space: nowrap; text-align: right">18.83 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">146.07 K</td>
    <td style="white-space: nowrap; text-align: right">6.85 μs</td>
    <td style="white-space: nowrap; text-align: right">±79.45%</td>
    <td style="white-space: nowrap; text-align: right">4.46 μs</td>
    <td style="white-space: nowrap; text-align: right">21.78 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">List.replace_at/3</td>
    <td style="white-space: nowrap; text-align: right">0.0875 K</td>
    <td style="white-space: nowrap; text-align: right">11432.23 μs</td>
    <td style="white-space: nowrap; text-align: right">±84.88%</td>
    <td style="white-space: nowrap; text-align: right">11965.16 μs</td>
    <td style="white-space: nowrap; text-align: right">39109.33 μs</td>
  </tr>

</table>


Comparison

<table style="width: 1%">
  <tr>
    <th>Name</th>
    <th style="text-align: right">IPS</th>
    <th style="text-align: right">Slower</th>
  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (RRBVector)</td>
    <td style="white-space: nowrap;text-align: right">219.93 K</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">206.73 K</td>
    <td style="white-space: nowrap; text-align: right">1.06x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">146.07 K</td>
    <td style="white-space: nowrap; text-align: right">1.51x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">List.replace_at/3</td>
    <td style="white-space: nowrap; text-align: right">0.0875 K</td>
    <td style="white-space: nowrap; text-align: right">2514.27x</td>
  </tr>

</table>



<hr/>

