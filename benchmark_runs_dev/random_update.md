
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
    <td style="white-space: nowrap; text-align: right">3.70 M</td>
    <td style="white-space: nowrap; text-align: right">270.20 ns</td>
    <td style="white-space: nowrap; text-align: right">±718.04%</td>
    <td style="white-space: nowrap; text-align: right">161 ns</td>
    <td style="white-space: nowrap; text-align: right">504.31 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">2.18 M</td>
    <td style="white-space: nowrap; text-align: right">457.71 ns</td>
    <td style="white-space: nowrap; text-align: right">±718.08%</td>
    <td style="white-space: nowrap; text-align: right">214 ns</td>
    <td style="white-space: nowrap; text-align: right">724.73 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">2.16 M</td>
    <td style="white-space: nowrap; text-align: right">462.99 ns</td>
    <td style="white-space: nowrap; text-align: right">±704.93%</td>
    <td style="white-space: nowrap; text-align: right">219 ns</td>
    <td style="white-space: nowrap; text-align: right">740.74 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.114 M</td>
    <td style="white-space: nowrap; text-align: right">8738.58 ns</td>
    <td style="white-space: nowrap; text-align: right">±47.07%</td>
    <td style="white-space: nowrap; text-align: right">7951 ns</td>
    <td style="white-space: nowrap; text-align: right">28611.24 ns</td>
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
    <td style="white-space: nowrap;text-align: right">3.70 M</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">2.18 M</td>
    <td style="white-space: nowrap; text-align: right">1.69x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">2.16 M</td>
    <td style="white-space: nowrap; text-align: right">1.71x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.114 M</td>
    <td style="white-space: nowrap; text-align: right">32.34x</td>
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
    <td style="white-space: nowrap; text-align: right">3.41 M</td>
    <td style="white-space: nowrap; text-align: right">293.26 ns</td>
    <td style="white-space: nowrap; text-align: right">±597.03%</td>
    <td style="white-space: nowrap; text-align: right">209 ns</td>
    <td style="white-space: nowrap; text-align: right">653.04 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">3.32 M</td>
    <td style="white-space: nowrap; text-align: right">301.10 ns</td>
    <td style="white-space: nowrap; text-align: right">±515.35%</td>
    <td style="white-space: nowrap; text-align: right">230 ns</td>
    <td style="white-space: nowrap; text-align: right">605.72 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">List.replace_at/3</td>
    <td style="white-space: nowrap; text-align: right">2.65 M</td>
    <td style="white-space: nowrap; text-align: right">377.55 ns</td>
    <td style="white-space: nowrap; text-align: right">±561.79%</td>
    <td style="white-space: nowrap; text-align: right">257 ns</td>
    <td style="white-space: nowrap; text-align: right">697 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.0948 M</td>
    <td style="white-space: nowrap; text-align: right">10545.31 ns</td>
    <td style="white-space: nowrap; text-align: right">±41.06%</td>
    <td style="white-space: nowrap; text-align: right">9726 ns</td>
    <td style="white-space: nowrap; text-align: right">31370.68 ns</td>
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
    <td style="white-space: nowrap;text-align: right">3.41 M</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">3.32 M</td>
    <td style="white-space: nowrap; text-align: right">1.03x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">List.replace_at/3</td>
    <td style="white-space: nowrap; text-align: right">2.65 M</td>
    <td style="white-space: nowrap; text-align: right">1.29x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.0948 M</td>
    <td style="white-space: nowrap; text-align: right">35.96x</td>
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
    <td style="white-space: nowrap">Arrays.replace/3 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">3.55 M</td>
    <td style="white-space: nowrap; text-align: right">282.01 ns</td>
    <td style="white-space: nowrap; text-align: right">±524.81%</td>
    <td style="white-space: nowrap; text-align: right">224 ns</td>
    <td style="white-space: nowrap; text-align: right">449.76 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">3.14 M</td>
    <td style="white-space: nowrap; text-align: right">318.15 ns</td>
    <td style="white-space: nowrap; text-align: right">±660.95%</td>
    <td style="white-space: nowrap; text-align: right">218 ns</td>
    <td style="white-space: nowrap; text-align: right">784 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">List.replace_at/3</td>
    <td style="white-space: nowrap; text-align: right">1.56 M</td>
    <td style="white-space: nowrap; text-align: right">640.46 ns</td>
    <td style="white-space: nowrap; text-align: right">±419.10%</td>
    <td style="white-space: nowrap; text-align: right">442 ns</td>
    <td style="white-space: nowrap; text-align: right">2710.80 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.0868 M</td>
    <td style="white-space: nowrap; text-align: right">11516.37 ns</td>
    <td style="white-space: nowrap; text-align: right">±44.15%</td>
    <td style="white-space: nowrap; text-align: right">10000 ns</td>
    <td style="white-space: nowrap; text-align: right">32816.30 ns</td>
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
    <td style="white-space: nowrap;text-align: right">3.55 M</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">3.14 M</td>
    <td style="white-space: nowrap; text-align: right">1.13x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">List.replace_at/3</td>
    <td style="white-space: nowrap; text-align: right">1.56 M</td>
    <td style="white-space: nowrap; text-align: right">2.27x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.0868 M</td>
    <td style="white-space: nowrap; text-align: right">40.84x</td>
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
    <td style="white-space: nowrap">Arrays.replace/3 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">3.15 M</td>
    <td style="white-space: nowrap; text-align: right">0.32 μs</td>
    <td style="white-space: nowrap; text-align: right">±523.21%</td>
    <td style="white-space: nowrap; text-align: right">0.24 μs</td>
    <td style="white-space: nowrap; text-align: right">0.46 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">3.10 M</td>
    <td style="white-space: nowrap; text-align: right">0.32 μs</td>
    <td style="white-space: nowrap; text-align: right">±571.10%</td>
    <td style="white-space: nowrap; text-align: right">0.24 μs</td>
    <td style="white-space: nowrap; text-align: right">0.66 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">List.replace_at/3</td>
    <td style="white-space: nowrap; text-align: right">0.91 M</td>
    <td style="white-space: nowrap; text-align: right">1.10 μs</td>
    <td style="white-space: nowrap; text-align: right">±308.07%</td>
    <td style="white-space: nowrap; text-align: right">0.77 μs</td>
    <td style="white-space: nowrap; text-align: right">13.87 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.0898 M</td>
    <td style="white-space: nowrap; text-align: right">11.13 μs</td>
    <td style="white-space: nowrap; text-align: right">±42.87%</td>
    <td style="white-space: nowrap; text-align: right">10.15 μs</td>
    <td style="white-space: nowrap; text-align: right">32.91 μs</td>
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
    <td style="white-space: nowrap;text-align: right">3.15 M</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">3.10 M</td>
    <td style="white-space: nowrap; text-align: right">1.02x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">List.replace_at/3</td>
    <td style="white-space: nowrap; text-align: right">0.91 M</td>
    <td style="white-space: nowrap; text-align: right">3.46x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.0898 M</td>
    <td style="white-space: nowrap; text-align: right">35.08x</td>
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
    <td style="white-space: nowrap; text-align: right">2.93 M</td>
    <td style="white-space: nowrap; text-align: right">0.34 μs</td>
    <td style="white-space: nowrap; text-align: right">±507.73%</td>
    <td style="white-space: nowrap; text-align: right">0.25 μs</td>
    <td style="white-space: nowrap; text-align: right">0.76 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">2.64 M</td>
    <td style="white-space: nowrap; text-align: right">0.38 μs</td>
    <td style="white-space: nowrap; text-align: right">±718.13%</td>
    <td style="white-space: nowrap; text-align: right">0.26 μs</td>
    <td style="white-space: nowrap; text-align: right">0.75 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">List.replace_at/3</td>
    <td style="white-space: nowrap; text-align: right">0.44 M</td>
    <td style="white-space: nowrap; text-align: right">2.25 μs</td>
    <td style="white-space: nowrap; text-align: right">±226.65%</td>
    <td style="white-space: nowrap; text-align: right">1.56 μs</td>
    <td style="white-space: nowrap; text-align: right">24.38 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.0811 M</td>
    <td style="white-space: nowrap; text-align: right">12.33 μs</td>
    <td style="white-space: nowrap; text-align: right">±43.70%</td>
    <td style="white-space: nowrap; text-align: right">11.35 μs</td>
    <td style="white-space: nowrap; text-align: right">37.92 μs</td>
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
    <td style="white-space: nowrap;text-align: right">2.93 M</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">2.64 M</td>
    <td style="white-space: nowrap; text-align: right">1.11x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">List.replace_at/3</td>
    <td style="white-space: nowrap; text-align: right">0.44 M</td>
    <td style="white-space: nowrap; text-align: right">6.6x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.0811 M</td>
    <td style="white-space: nowrap; text-align: right">36.16x</td>
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
    <td style="white-space: nowrap; text-align: right">2.88 M</td>
    <td style="white-space: nowrap; text-align: right">0.35 μs</td>
    <td style="white-space: nowrap; text-align: right">±587.41%</td>
    <td style="white-space: nowrap; text-align: right">0.26 μs</td>
    <td style="white-space: nowrap; text-align: right">0.62 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">2.87 M</td>
    <td style="white-space: nowrap; text-align: right">0.35 μs</td>
    <td style="white-space: nowrap; text-align: right">±483.33%</td>
    <td style="white-space: nowrap; text-align: right">0.26 μs</td>
    <td style="white-space: nowrap; text-align: right">0.72 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">List.replace_at/3</td>
    <td style="white-space: nowrap; text-align: right">0.22 M</td>
    <td style="white-space: nowrap; text-align: right">4.55 μs</td>
    <td style="white-space: nowrap; text-align: right">±173.88%</td>
    <td style="white-space: nowrap; text-align: right">3.11 μs</td>
    <td style="white-space: nowrap; text-align: right">51.81 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.0764 M</td>
    <td style="white-space: nowrap; text-align: right">13.08 μs</td>
    <td style="white-space: nowrap; text-align: right">±39.82%</td>
    <td style="white-space: nowrap; text-align: right">12.25 μs</td>
    <td style="white-space: nowrap; text-align: right">41.74 μs</td>
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
    <td style="white-space: nowrap;text-align: right">2.88 M</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">2.87 M</td>
    <td style="white-space: nowrap; text-align: right">1.0x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">List.replace_at/3</td>
    <td style="white-space: nowrap; text-align: right">0.22 M</td>
    <td style="white-space: nowrap; text-align: right">13.08x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.0764 M</td>
    <td style="white-space: nowrap; text-align: right">37.63x</td>
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
    <td style="white-space: nowrap; text-align: right">2.81 M</td>
    <td style="white-space: nowrap; text-align: right">0.36 μs</td>
    <td style="white-space: nowrap; text-align: right">±451.11%</td>
    <td style="white-space: nowrap; text-align: right">0.28 μs</td>
    <td style="white-space: nowrap; text-align: right">0.64 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">2.65 M</td>
    <td style="white-space: nowrap; text-align: right">0.38 μs</td>
    <td style="white-space: nowrap; text-align: right">±514.35%</td>
    <td style="white-space: nowrap; text-align: right">0.28 μs</td>
    <td style="white-space: nowrap; text-align: right">0.77 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">List.replace_at/3</td>
    <td style="white-space: nowrap; text-align: right">0.112 M</td>
    <td style="white-space: nowrap; text-align: right">8.95 μs</td>
    <td style="white-space: nowrap; text-align: right">±133.78%</td>
    <td style="white-space: nowrap; text-align: right">6.22 μs</td>
    <td style="white-space: nowrap; text-align: right">63.82 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.0663 M</td>
    <td style="white-space: nowrap; text-align: right">15.08 μs</td>
    <td style="white-space: nowrap; text-align: right">±40.60%</td>
    <td style="white-space: nowrap; text-align: right">14.01 μs</td>
    <td style="white-space: nowrap; text-align: right">49.35 μs</td>
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
    <td style="white-space: nowrap;text-align: right">2.81 M</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">2.65 M</td>
    <td style="white-space: nowrap; text-align: right">1.06x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">List.replace_at/3</td>
    <td style="white-space: nowrap; text-align: right">0.112 M</td>
    <td style="white-space: nowrap; text-align: right">25.1x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.0663 M</td>
    <td style="white-space: nowrap; text-align: right">42.31x</td>
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
    <td style="white-space: nowrap; text-align: right">2.37 M</td>
    <td style="white-space: nowrap; text-align: right">0.42 μs</td>
    <td style="white-space: nowrap; text-align: right">±449.61%</td>
    <td style="white-space: nowrap; text-align: right">0.32 μs</td>
    <td style="white-space: nowrap; text-align: right">1.53 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">2.13 M</td>
    <td style="white-space: nowrap; text-align: right">0.47 μs</td>
    <td style="white-space: nowrap; text-align: right">±617.29%</td>
    <td style="white-space: nowrap; text-align: right">0.32 μs</td>
    <td style="white-space: nowrap; text-align: right">0.76 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.0567 M</td>
    <td style="white-space: nowrap; text-align: right">17.64 μs</td>
    <td style="white-space: nowrap; text-align: right">±40.45%</td>
    <td style="white-space: nowrap; text-align: right">16.33 μs</td>
    <td style="white-space: nowrap; text-align: right">52.81 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">List.replace_at/3</td>
    <td style="white-space: nowrap; text-align: right">0.0566 M</td>
    <td style="white-space: nowrap; text-align: right">17.66 μs</td>
    <td style="white-space: nowrap; text-align: right">±105.84%</td>
    <td style="white-space: nowrap; text-align: right">12.49 μs</td>
    <td style="white-space: nowrap; text-align: right">78.93 μs</td>
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
    <td style="white-space: nowrap;text-align: right">2.37 M</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">2.13 M</td>
    <td style="white-space: nowrap; text-align: right">1.11x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.0567 M</td>
    <td style="white-space: nowrap; text-align: right">41.75x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">List.replace_at/3</td>
    <td style="white-space: nowrap; text-align: right">0.0566 M</td>
    <td style="white-space: nowrap; text-align: right">41.81x</td>
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
    <td style="white-space: nowrap">Arrays.replace/3 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">2.48 M</td>
    <td style="white-space: nowrap; text-align: right">0.40 μs</td>
    <td style="white-space: nowrap; text-align: right">±181.53%</td>
    <td style="white-space: nowrap; text-align: right">0.35 μs</td>
    <td style="white-space: nowrap; text-align: right">0.67 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">2.15 M</td>
    <td style="white-space: nowrap; text-align: right">0.47 μs</td>
    <td style="white-space: nowrap; text-align: right">±561.32%</td>
    <td style="white-space: nowrap; text-align: right">0.35 μs</td>
    <td style="white-space: nowrap; text-align: right">0.78 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.0520 M</td>
    <td style="white-space: nowrap; text-align: right">19.23 μs</td>
    <td style="white-space: nowrap; text-align: right">±37.84%</td>
    <td style="white-space: nowrap; text-align: right">17.61 μs</td>
    <td style="white-space: nowrap; text-align: right">54.32 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">List.replace_at/3</td>
    <td style="white-space: nowrap; text-align: right">0.0292 M</td>
    <td style="white-space: nowrap; text-align: right">34.21 μs</td>
    <td style="white-space: nowrap; text-align: right">±87.96%</td>
    <td style="white-space: nowrap; text-align: right">26.11 μs</td>
    <td style="white-space: nowrap; text-align: right">134.33 μs</td>
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
    <td style="white-space: nowrap;text-align: right">2.48 M</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">2.15 M</td>
    <td style="white-space: nowrap; text-align: right">1.16x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.0520 M</td>
    <td style="white-space: nowrap; text-align: right">47.77x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">List.replace_at/3</td>
    <td style="white-space: nowrap; text-align: right">0.0292 M</td>
    <td style="white-space: nowrap; text-align: right">84.99x</td>
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
    <td style="white-space: nowrap">Arrays.replace/3 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">2.24 M</td>
    <td style="white-space: nowrap; text-align: right">0.45 μs</td>
    <td style="white-space: nowrap; text-align: right">±133.41%</td>
    <td style="white-space: nowrap; text-align: right">0.37 μs</td>
    <td style="white-space: nowrap; text-align: right">0.73 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">2.06 M</td>
    <td style="white-space: nowrap; text-align: right">0.49 μs</td>
    <td style="white-space: nowrap; text-align: right">±188.29%</td>
    <td style="white-space: nowrap; text-align: right">0.38 μs</td>
    <td style="white-space: nowrap; text-align: right">1.02 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.0521 M</td>
    <td style="white-space: nowrap; text-align: right">19.21 μs</td>
    <td style="white-space: nowrap; text-align: right">±34.27%</td>
    <td style="white-space: nowrap; text-align: right">17.75 μs</td>
    <td style="white-space: nowrap; text-align: right">53.43 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">List.replace_at/3</td>
    <td style="white-space: nowrap; text-align: right">0.0150 M</td>
    <td style="white-space: nowrap; text-align: right">66.76 μs</td>
    <td style="white-space: nowrap; text-align: right">±79.49%</td>
    <td style="white-space: nowrap; text-align: right">56.99 μs</td>
    <td style="white-space: nowrap; text-align: right">233.72 μs</td>
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
    <td style="white-space: nowrap;text-align: right">2.24 M</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">2.06 M</td>
    <td style="white-space: nowrap; text-align: right">1.09x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.0521 M</td>
    <td style="white-space: nowrap; text-align: right">43.12x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">List.replace_at/3</td>
    <td style="white-space: nowrap; text-align: right">0.0150 M</td>
    <td style="white-space: nowrap; text-align: right">149.83x</td>
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
    <td style="white-space: nowrap; text-align: right">915.34 K</td>
    <td style="white-space: nowrap; text-align: right">1.09 μs</td>
    <td style="white-space: nowrap; text-align: right">±1512.96%</td>
    <td style="white-space: nowrap; text-align: right">0.42 μs</td>
    <td style="white-space: nowrap; text-align: right">1.42 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">559.15 K</td>
    <td style="white-space: nowrap; text-align: right">1.79 μs</td>
    <td style="white-space: nowrap; text-align: right">±1485.55%</td>
    <td style="white-space: nowrap; text-align: right">0.41 μs</td>
    <td style="white-space: nowrap; text-align: right">1.61 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">50.22 K</td>
    <td style="white-space: nowrap; text-align: right">19.91 μs</td>
    <td style="white-space: nowrap; text-align: right">±34.01%</td>
    <td style="white-space: nowrap; text-align: right">18.32 μs</td>
    <td style="white-space: nowrap; text-align: right">54.03 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">List.replace_at/3</td>
    <td style="white-space: nowrap; text-align: right">5.56 K</td>
    <td style="white-space: nowrap; text-align: right">179.73 μs</td>
    <td style="white-space: nowrap; text-align: right">±91.26%</td>
    <td style="white-space: nowrap; text-align: right">125.73 μs</td>
    <td style="white-space: nowrap; text-align: right">671.38 μs</td>
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
    <td style="white-space: nowrap;text-align: right">915.34 K</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">559.15 K</td>
    <td style="white-space: nowrap; text-align: right">1.64x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">50.22 K</td>
    <td style="white-space: nowrap; text-align: right">18.23x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">List.replace_at/3</td>
    <td style="white-space: nowrap; text-align: right">5.56 K</td>
    <td style="white-space: nowrap; text-align: right">164.51x</td>
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
    <td style="white-space: nowrap">Arrays.replace/3 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">369.90 K</td>
    <td style="white-space: nowrap; text-align: right">2.70 μs</td>
    <td style="white-space: nowrap; text-align: right">±101.53%</td>
    <td style="white-space: nowrap; text-align: right">2.38 μs</td>
    <td style="white-space: nowrap; text-align: right">17.88 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">365.74 K</td>
    <td style="white-space: nowrap; text-align: right">2.73 μs</td>
    <td style="white-space: nowrap; text-align: right">±101.81%</td>
    <td style="white-space: nowrap; text-align: right">2.40 μs</td>
    <td style="white-space: nowrap; text-align: right">18.59 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">46.69 K</td>
    <td style="white-space: nowrap; text-align: right">21.42 μs</td>
    <td style="white-space: nowrap; text-align: right">±35.10%</td>
    <td style="white-space: nowrap; text-align: right">19.63 μs</td>
    <td style="white-space: nowrap; text-align: right">56.80 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">List.replace_at/3</td>
    <td style="white-space: nowrap; text-align: right">2.93 K</td>
    <td style="white-space: nowrap; text-align: right">341.49 μs</td>
    <td style="white-space: nowrap; text-align: right">±79.89%</td>
    <td style="white-space: nowrap; text-align: right">260.50 μs</td>
    <td style="white-space: nowrap; text-align: right">1031.67 μs</td>
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
    <td style="white-space: nowrap;text-align: right">369.90 K</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">365.74 K</td>
    <td style="white-space: nowrap; text-align: right">1.01x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">46.69 K</td>
    <td style="white-space: nowrap; text-align: right">7.92x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">List.replace_at/3</td>
    <td style="white-space: nowrap; text-align: right">2.93 K</td>
    <td style="white-space: nowrap; text-align: right">126.32x</td>
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
    <td style="white-space: nowrap">Arrays.replace/3 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">269.77 K</td>
    <td style="white-space: nowrap; text-align: right">3.71 μs</td>
    <td style="white-space: nowrap; text-align: right">±52.03%</td>
    <td style="white-space: nowrap; text-align: right">3.46 μs</td>
    <td style="white-space: nowrap; text-align: right">17.25 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">236.31 K</td>
    <td style="white-space: nowrap; text-align: right">4.23 μs</td>
    <td style="white-space: nowrap; text-align: right">±79.18%</td>
    <td style="white-space: nowrap; text-align: right">3.52 μs</td>
    <td style="white-space: nowrap; text-align: right">23.38 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">43.55 K</td>
    <td style="white-space: nowrap; text-align: right">22.96 μs</td>
    <td style="white-space: nowrap; text-align: right">±33.17%</td>
    <td style="white-space: nowrap; text-align: right">21.07 μs</td>
    <td style="white-space: nowrap; text-align: right">57.59 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">List.replace_at/3</td>
    <td style="white-space: nowrap; text-align: right">1.24 K</td>
    <td style="white-space: nowrap; text-align: right">805.59 μs</td>
    <td style="white-space: nowrap; text-align: right">±107.27%</td>
    <td style="white-space: nowrap; text-align: right">610.23 μs</td>
    <td style="white-space: nowrap; text-align: right">3975.83 μs</td>
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
    <td style="white-space: nowrap;text-align: right">269.77 K</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">236.31 K</td>
    <td style="white-space: nowrap; text-align: right">1.14x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">43.55 K</td>
    <td style="white-space: nowrap; text-align: right">6.19x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">List.replace_at/3</td>
    <td style="white-space: nowrap; text-align: right">1.24 K</td>
    <td style="white-space: nowrap; text-align: right">217.33x</td>
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
    <td style="white-space: nowrap">Arrays.replace/3 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">281.63 K</td>
    <td style="white-space: nowrap; text-align: right">3.55 μs</td>
    <td style="white-space: nowrap; text-align: right">±25.00%</td>
    <td style="white-space: nowrap; text-align: right">3.39 μs</td>
    <td style="white-space: nowrap; text-align: right">10.24 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">267.31 K</td>
    <td style="white-space: nowrap; text-align: right">3.74 μs</td>
    <td style="white-space: nowrap; text-align: right">±22.92%</td>
    <td style="white-space: nowrap; text-align: right">3.61 μs</td>
    <td style="white-space: nowrap; text-align: right">11.29 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">38.59 K</td>
    <td style="white-space: nowrap; text-align: right">25.91 μs</td>
    <td style="white-space: nowrap; text-align: right">±35.63%</td>
    <td style="white-space: nowrap; text-align: right">23.53 μs</td>
    <td style="white-space: nowrap; text-align: right">67.03 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">List.replace_at/3</td>
    <td style="white-space: nowrap; text-align: right">0.55 K</td>
    <td style="white-space: nowrap; text-align: right">1806.46 μs</td>
    <td style="white-space: nowrap; text-align: right">±94.46%</td>
    <td style="white-space: nowrap; text-align: right">1101.97 μs</td>
    <td style="white-space: nowrap; text-align: right">5658.09 μs</td>
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
    <td style="white-space: nowrap;text-align: right">281.63 K</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">267.31 K</td>
    <td style="white-space: nowrap; text-align: right">1.05x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">38.59 K</td>
    <td style="white-space: nowrap; text-align: right">7.3x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">List.replace_at/3</td>
    <td style="white-space: nowrap; text-align: right">0.55 K</td>
    <td style="white-space: nowrap; text-align: right">508.76x</td>
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
    <td style="white-space: nowrap">Arrays.replace/3 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">262.43 K</td>
    <td style="white-space: nowrap; text-align: right">3.81 μs</td>
    <td style="white-space: nowrap; text-align: right">±55.00%</td>
    <td style="white-space: nowrap; text-align: right">3.32 μs</td>
    <td style="white-space: nowrap; text-align: right">17.71 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">226.90 K</td>
    <td style="white-space: nowrap; text-align: right">4.41 μs</td>
    <td style="white-space: nowrap; text-align: right">±92.88%</td>
    <td style="white-space: nowrap; text-align: right">3.33 μs</td>
    <td style="white-space: nowrap; text-align: right">24.70 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">36.92 K</td>
    <td style="white-space: nowrap; text-align: right">27.08 μs</td>
    <td style="white-space: nowrap; text-align: right">±38.92%</td>
    <td style="white-space: nowrap; text-align: right">24.45 μs</td>
    <td style="white-space: nowrap; text-align: right">76.78 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">List.replace_at/3</td>
    <td style="white-space: nowrap; text-align: right">0.173 K</td>
    <td style="white-space: nowrap; text-align: right">5790.54 μs</td>
    <td style="white-space: nowrap; text-align: right">±85.11%</td>
    <td style="white-space: nowrap; text-align: right">5468.72 μs</td>
    <td style="white-space: nowrap; text-align: right">26792.57 μs</td>
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
    <td style="white-space: nowrap;text-align: right">262.43 K</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">226.90 K</td>
    <td style="white-space: nowrap; text-align: right">1.16x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">36.92 K</td>
    <td style="white-space: nowrap; text-align: right">7.11x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">List.replace_at/3</td>
    <td style="white-space: nowrap; text-align: right">0.173 K</td>
    <td style="white-space: nowrap; text-align: right">1519.61x</td>
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
    <td style="white-space: nowrap">Arrays.replace/3 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">189.91 K</td>
    <td style="white-space: nowrap; text-align: right">5.27 μs</td>
    <td style="white-space: nowrap; text-align: right">±78.46%</td>
    <td style="white-space: nowrap; text-align: right">4.34 μs</td>
    <td style="white-space: nowrap; text-align: right">23.34 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">187.45 K</td>
    <td style="white-space: nowrap; text-align: right">5.33 μs</td>
    <td style="white-space: nowrap; text-align: right">±75.61%</td>
    <td style="white-space: nowrap; text-align: right">4.09 μs</td>
    <td style="white-space: nowrap; text-align: right">18.27 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">35.18 K</td>
    <td style="white-space: nowrap; text-align: right">28.42 μs</td>
    <td style="white-space: nowrap; text-align: right">±36.33%</td>
    <td style="white-space: nowrap; text-align: right">26.02 μs</td>
    <td style="white-space: nowrap; text-align: right">69.99 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">List.replace_at/3</td>
    <td style="white-space: nowrap; text-align: right">0.0871 K</td>
    <td style="white-space: nowrap; text-align: right">11477.68 μs</td>
    <td style="white-space: nowrap; text-align: right">±96.36%</td>
    <td style="white-space: nowrap; text-align: right">7090.41 μs</td>
    <td style="white-space: nowrap; text-align: right">36377.64 μs</td>
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
    <td style="white-space: nowrap;text-align: right">189.91 K</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">187.45 K</td>
    <td style="white-space: nowrap; text-align: right">1.01x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">35.18 K</td>
    <td style="white-space: nowrap; text-align: right">5.4x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">List.replace_at/3</td>
    <td style="white-space: nowrap; text-align: right">0.0871 K</td>
    <td style="white-space: nowrap; text-align: right">2179.75x</td>
  </tr>

</table>



<hr/>

