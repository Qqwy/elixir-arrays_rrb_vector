
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
    <td style="white-space: nowrap; text-align: right">3.66 M</td>
    <td style="white-space: nowrap; text-align: right">273.27 ns</td>
    <td style="white-space: nowrap; text-align: right">±690.14%</td>
    <td style="white-space: nowrap; text-align: right">165 ns</td>
    <td style="white-space: nowrap; text-align: right">547 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">2.19 M</td>
    <td style="white-space: nowrap; text-align: right">457.55 ns</td>
    <td style="white-space: nowrap; text-align: right">±637.83%</td>
    <td style="white-space: nowrap; text-align: right">221 ns</td>
    <td style="white-space: nowrap; text-align: right">771.04 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">2.05 M</td>
    <td style="white-space: nowrap; text-align: right">488.22 ns</td>
    <td style="white-space: nowrap; text-align: right">±708.96%</td>
    <td style="white-space: nowrap; text-align: right">224 ns</td>
    <td style="white-space: nowrap; text-align: right">901.77 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.48 M</td>
    <td style="white-space: nowrap; text-align: right">2082.08 ns</td>
    <td style="white-space: nowrap; text-align: right">±140.85%</td>
    <td style="white-space: nowrap; text-align: right">1842 ns</td>
    <td style="white-space: nowrap; text-align: right">15290.75 ns</td>
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
    <td style="white-space: nowrap;text-align: right">3.66 M</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">2.19 M</td>
    <td style="white-space: nowrap; text-align: right">1.67x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">2.05 M</td>
    <td style="white-space: nowrap; text-align: right">1.79x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.48 M</td>
    <td style="white-space: nowrap; text-align: right">7.62x</td>
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
    <td style="white-space: nowrap; text-align: right">3.50 M</td>
    <td style="white-space: nowrap; text-align: right">286.06 ns</td>
    <td style="white-space: nowrap; text-align: right">±487.25%</td>
    <td style="white-space: nowrap; text-align: right">218 ns</td>
    <td style="white-space: nowrap; text-align: right">645 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">3.19 M</td>
    <td style="white-space: nowrap; text-align: right">313.56 ns</td>
    <td style="white-space: nowrap; text-align: right">±522.61%</td>
    <td style="white-space: nowrap; text-align: right">224 ns</td>
    <td style="white-space: nowrap; text-align: right">987 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">List.replace_at/3</td>
    <td style="white-space: nowrap; text-align: right">2.37 M</td>
    <td style="white-space: nowrap; text-align: right">421.07 ns</td>
    <td style="white-space: nowrap; text-align: right">±566.63%</td>
    <td style="white-space: nowrap; text-align: right">264 ns</td>
    <td style="white-space: nowrap; text-align: right">762 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.45 M</td>
    <td style="white-space: nowrap; text-align: right">2232.55 ns</td>
    <td style="white-space: nowrap; text-align: right">±125.79%</td>
    <td style="white-space: nowrap; text-align: right">1967 ns</td>
    <td style="white-space: nowrap; text-align: right">17982.84 ns</td>
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
    <td style="white-space: nowrap;text-align: right">3.50 M</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">3.19 M</td>
    <td style="white-space: nowrap; text-align: right">1.1x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">List.replace_at/3</td>
    <td style="white-space: nowrap; text-align: right">2.37 M</td>
    <td style="white-space: nowrap; text-align: right">1.47x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.45 M</td>
    <td style="white-space: nowrap; text-align: right">7.8x</td>
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
    <td style="white-space: nowrap; text-align: right">3.41 M</td>
    <td style="white-space: nowrap; text-align: right">293.45 ns</td>
    <td style="white-space: nowrap; text-align: right">±358.90%</td>
    <td style="white-space: nowrap; text-align: right">233 ns</td>
    <td style="white-space: nowrap; text-align: right">686.56 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">3.17 M</td>
    <td style="white-space: nowrap; text-align: right">315.39 ns</td>
    <td style="white-space: nowrap; text-align: right">±494.95%</td>
    <td style="white-space: nowrap; text-align: right">231 ns</td>
    <td style="white-space: nowrap; text-align: right">689.39 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">List.replace_at/3</td>
    <td style="white-space: nowrap; text-align: right">1.55 M</td>
    <td style="white-space: nowrap; text-align: right">646.75 ns</td>
    <td style="white-space: nowrap; text-align: right">±410.01%</td>
    <td style="white-space: nowrap; text-align: right">443 ns</td>
    <td style="white-space: nowrap; text-align: right">2563.70 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.43 M</td>
    <td style="white-space: nowrap; text-align: right">2332.51 ns</td>
    <td style="white-space: nowrap; text-align: right">±126.28%</td>
    <td style="white-space: nowrap; text-align: right">1998 ns</td>
    <td style="white-space: nowrap; text-align: right">15020.63 ns</td>
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
    <td style="white-space: nowrap; text-align: right">3.17 M</td>
    <td style="white-space: nowrap; text-align: right">1.07x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">List.replace_at/3</td>
    <td style="white-space: nowrap; text-align: right">1.55 M</td>
    <td style="white-space: nowrap; text-align: right">2.2x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.43 M</td>
    <td style="white-space: nowrap; text-align: right">7.95x</td>
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
    <td style="white-space: nowrap; text-align: right">2.97 M</td>
    <td style="white-space: nowrap; text-align: right">0.34 μs</td>
    <td style="white-space: nowrap; text-align: right">±463.89%</td>
    <td style="white-space: nowrap; text-align: right">0.25 μs</td>
    <td style="white-space: nowrap; text-align: right">0.69 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">2.94 M</td>
    <td style="white-space: nowrap; text-align: right">0.34 μs</td>
    <td style="white-space: nowrap; text-align: right">±510.44%</td>
    <td style="white-space: nowrap; text-align: right">0.25 μs</td>
    <td style="white-space: nowrap; text-align: right">0.70 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">List.replace_at/3</td>
    <td style="white-space: nowrap; text-align: right">0.86 M</td>
    <td style="white-space: nowrap; text-align: right">1.17 μs</td>
    <td style="white-space: nowrap; text-align: right">±313.40%</td>
    <td style="white-space: nowrap; text-align: right">0.80 μs</td>
    <td style="white-space: nowrap; text-align: right">13.96 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.42 M</td>
    <td style="white-space: nowrap; text-align: right">2.37 μs</td>
    <td style="white-space: nowrap; text-align: right">±128.97%</td>
    <td style="white-space: nowrap; text-align: right">2.00 μs</td>
    <td style="white-space: nowrap; text-align: right">16.41 μs</td>
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
    <td style="white-space: nowrap;text-align: right">2.97 M</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">2.94 M</td>
    <td style="white-space: nowrap; text-align: right">1.01x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">List.replace_at/3</td>
    <td style="white-space: nowrap; text-align: right">0.86 M</td>
    <td style="white-space: nowrap; text-align: right">3.46x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.42 M</td>
    <td style="white-space: nowrap; text-align: right">7.03x</td>
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
    <td style="white-space: nowrap; text-align: right">2.63 M</td>
    <td style="white-space: nowrap; text-align: right">0.38 μs</td>
    <td style="white-space: nowrap; text-align: right">±615.37%</td>
    <td style="white-space: nowrap; text-align: right">0.27 μs</td>
    <td style="white-space: nowrap; text-align: right">0.79 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">2.59 M</td>
    <td style="white-space: nowrap; text-align: right">0.39 μs</td>
    <td style="white-space: nowrap; text-align: right">±532.36%</td>
    <td style="white-space: nowrap; text-align: right">0.27 μs</td>
    <td style="white-space: nowrap; text-align: right">0.84 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">List.replace_at/3</td>
    <td style="white-space: nowrap; text-align: right">0.46 M</td>
    <td style="white-space: nowrap; text-align: right">2.16 μs</td>
    <td style="white-space: nowrap; text-align: right">±226.90%</td>
    <td style="white-space: nowrap; text-align: right">1.50 μs</td>
    <td style="white-space: nowrap; text-align: right">24.54 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.41 M</td>
    <td style="white-space: nowrap; text-align: right">2.44 μs</td>
    <td style="white-space: nowrap; text-align: right">±124.46%</td>
    <td style="white-space: nowrap; text-align: right">2.06 μs</td>
    <td style="white-space: nowrap; text-align: right">17.49 μs</td>
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
    <td style="white-space: nowrap;text-align: right">2.63 M</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">2.59 M</td>
    <td style="white-space: nowrap; text-align: right">1.02x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">List.replace_at/3</td>
    <td style="white-space: nowrap; text-align: right">0.46 M</td>
    <td style="white-space: nowrap; text-align: right">5.69x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.41 M</td>
    <td style="white-space: nowrap; text-align: right">6.42x</td>
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
    <td style="white-space: nowrap; text-align: right">2.73 M</td>
    <td style="white-space: nowrap; text-align: right">0.37 μs</td>
    <td style="white-space: nowrap; text-align: right">±507.11%</td>
    <td style="white-space: nowrap; text-align: right">0.26 μs</td>
    <td style="white-space: nowrap; text-align: right">1.02 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">2.63 M</td>
    <td style="white-space: nowrap; text-align: right">0.38 μs</td>
    <td style="white-space: nowrap; text-align: right">±508.31%</td>
    <td style="white-space: nowrap; text-align: right">0.27 μs</td>
    <td style="white-space: nowrap; text-align: right">1.00 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.39 M</td>
    <td style="white-space: nowrap; text-align: right">2.55 μs</td>
    <td style="white-space: nowrap; text-align: right">±118.36%</td>
    <td style="white-space: nowrap; text-align: right">2.14 μs</td>
    <td style="white-space: nowrap; text-align: right">18.26 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">List.replace_at/3</td>
    <td style="white-space: nowrap; text-align: right">0.23 M</td>
    <td style="white-space: nowrap; text-align: right">4.44 μs</td>
    <td style="white-space: nowrap; text-align: right">±170.08%</td>
    <td style="white-space: nowrap; text-align: right">3.09 μs</td>
    <td style="white-space: nowrap; text-align: right">41.27 μs</td>
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
    <td style="white-space: nowrap;text-align: right">2.73 M</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">2.63 M</td>
    <td style="white-space: nowrap; text-align: right">1.04x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.39 M</td>
    <td style="white-space: nowrap; text-align: right">6.97x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">List.replace_at/3</td>
    <td style="white-space: nowrap; text-align: right">0.23 M</td>
    <td style="white-space: nowrap; text-align: right">12.12x</td>
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
    <td style="white-space: nowrap">Arrays.replace/3 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">2.46 M</td>
    <td style="white-space: nowrap; text-align: right">0.41 μs</td>
    <td style="white-space: nowrap; text-align: right">±511.51%</td>
    <td style="white-space: nowrap; text-align: right">0.29 μs</td>
    <td style="white-space: nowrap; text-align: right">1.06 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">2.42 M</td>
    <td style="white-space: nowrap; text-align: right">0.41 μs</td>
    <td style="white-space: nowrap; text-align: right">±476.79%</td>
    <td style="white-space: nowrap; text-align: right">0.29 μs</td>
    <td style="white-space: nowrap; text-align: right">1.08 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.37 M</td>
    <td style="white-space: nowrap; text-align: right">2.71 μs</td>
    <td style="white-space: nowrap; text-align: right">±112.58%</td>
    <td style="white-space: nowrap; text-align: right">2.38 μs</td>
    <td style="white-space: nowrap; text-align: right">18.85 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">List.replace_at/3</td>
    <td style="white-space: nowrap; text-align: right">0.115 M</td>
    <td style="white-space: nowrap; text-align: right">8.73 μs</td>
    <td style="white-space: nowrap; text-align: right">±137.79%</td>
    <td style="white-space: nowrap; text-align: right">6.05 μs</td>
    <td style="white-space: nowrap; text-align: right">63.52 μs</td>
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
    <td style="white-space: nowrap;text-align: right">2.46 M</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">2.42 M</td>
    <td style="white-space: nowrap; text-align: right">1.02x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.37 M</td>
    <td style="white-space: nowrap; text-align: right">6.67x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">List.replace_at/3</td>
    <td style="white-space: nowrap; text-align: right">0.115 M</td>
    <td style="white-space: nowrap; text-align: right">21.51x</td>
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
    <td style="white-space: nowrap; text-align: right">2.25 M</td>
    <td style="white-space: nowrap; text-align: right">0.45 μs</td>
    <td style="white-space: nowrap; text-align: right">±444.25%</td>
    <td style="white-space: nowrap; text-align: right">0.33 μs</td>
    <td style="white-space: nowrap; text-align: right">1.13 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">2.10 M</td>
    <td style="white-space: nowrap; text-align: right">0.48 μs</td>
    <td style="white-space: nowrap; text-align: right">±539.00%</td>
    <td style="white-space: nowrap; text-align: right">0.33 μs</td>
    <td style="white-space: nowrap; text-align: right">1.13 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.32 M</td>
    <td style="white-space: nowrap; text-align: right">3.08 μs</td>
    <td style="white-space: nowrap; text-align: right">±100.24%</td>
    <td style="white-space: nowrap; text-align: right">2.77 μs</td>
    <td style="white-space: nowrap; text-align: right">19.20 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">List.replace_at/3</td>
    <td style="white-space: nowrap; text-align: right">0.0572 M</td>
    <td style="white-space: nowrap; text-align: right">17.49 μs</td>
    <td style="white-space: nowrap; text-align: right">±108.96%</td>
    <td style="white-space: nowrap; text-align: right">12.37 μs</td>
    <td style="white-space: nowrap; text-align: right">80.76 μs</td>
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
    <td style="white-space: nowrap;text-align: right">2.25 M</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">2.10 M</td>
    <td style="white-space: nowrap; text-align: right">1.07x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.32 M</td>
    <td style="white-space: nowrap; text-align: right">6.91x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">List.replace_at/3</td>
    <td style="white-space: nowrap; text-align: right">0.0572 M</td>
    <td style="white-space: nowrap; text-align: right">39.3x</td>
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
    <td style="white-space: nowrap; text-align: right">2.04 M</td>
    <td style="white-space: nowrap; text-align: right">0.49 μs</td>
    <td style="white-space: nowrap; text-align: right">±200.00%</td>
    <td style="white-space: nowrap; text-align: right">0.37 μs</td>
    <td style="white-space: nowrap; text-align: right">1.18 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">2.00 M</td>
    <td style="white-space: nowrap; text-align: right">0.50 μs</td>
    <td style="white-space: nowrap; text-align: right">±201.22%</td>
    <td style="white-space: nowrap; text-align: right">0.37 μs</td>
    <td style="white-space: nowrap; text-align: right">1.29 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.31 M</td>
    <td style="white-space: nowrap; text-align: right">3.19 μs</td>
    <td style="white-space: nowrap; text-align: right">±103.06%</td>
    <td style="white-space: nowrap; text-align: right">2.86 μs</td>
    <td style="white-space: nowrap; text-align: right">19.06 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">List.replace_at/3</td>
    <td style="white-space: nowrap; text-align: right">0.0300 M</td>
    <td style="white-space: nowrap; text-align: right">33.31 μs</td>
    <td style="white-space: nowrap; text-align: right">±89.49%</td>
    <td style="white-space: nowrap; text-align: right">25.23 μs</td>
    <td style="white-space: nowrap; text-align: right">135.25 μs</td>
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
    <td style="white-space: nowrap;text-align: right">2.04 M</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">2.00 M</td>
    <td style="white-space: nowrap; text-align: right">1.02x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.31 M</td>
    <td style="white-space: nowrap; text-align: right">6.51x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">List.replace_at/3</td>
    <td style="white-space: nowrap; text-align: right">0.0300 M</td>
    <td style="white-space: nowrap; text-align: right">67.95x</td>
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
    <td style="white-space: nowrap; text-align: right">1.77 M</td>
    <td style="white-space: nowrap; text-align: right">0.57 μs</td>
    <td style="white-space: nowrap; text-align: right">±201.19%</td>
    <td style="white-space: nowrap; text-align: right">0.39 μs</td>
    <td style="white-space: nowrap; text-align: right">1.59 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">1.75 M</td>
    <td style="white-space: nowrap; text-align: right">0.57 μs</td>
    <td style="white-space: nowrap; text-align: right">±184.52%</td>
    <td style="white-space: nowrap; text-align: right">0.40 μs</td>
    <td style="white-space: nowrap; text-align: right">1.54 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.30 M</td>
    <td style="white-space: nowrap; text-align: right">3.37 μs</td>
    <td style="white-space: nowrap; text-align: right">±90.95%</td>
    <td style="white-space: nowrap; text-align: right">3.01 μs</td>
    <td style="white-space: nowrap; text-align: right">19.69 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">List.replace_at/3</td>
    <td style="white-space: nowrap; text-align: right">0.0150 M</td>
    <td style="white-space: nowrap; text-align: right">66.48 μs</td>
    <td style="white-space: nowrap; text-align: right">±80.08%</td>
    <td style="white-space: nowrap; text-align: right">55.79 μs</td>
    <td style="white-space: nowrap; text-align: right">236.19 μs</td>
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
    <td style="white-space: nowrap;text-align: right">1.77 M</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">1.75 M</td>
    <td style="white-space: nowrap; text-align: right">1.01x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.30 M</td>
    <td style="white-space: nowrap; text-align: right">5.95x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">List.replace_at/3</td>
    <td style="white-space: nowrap; text-align: right">0.0150 M</td>
    <td style="white-space: nowrap; text-align: right">117.37x</td>
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
    <td style="white-space: nowrap; text-align: right">538.24 K</td>
    <td style="white-space: nowrap; text-align: right">1.86 μs</td>
    <td style="white-space: nowrap; text-align: right">±1266.40%</td>
    <td style="white-space: nowrap; text-align: right">0.58 μs</td>
    <td style="white-space: nowrap; text-align: right">2.59 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">377.42 K</td>
    <td style="white-space: nowrap; text-align: right">2.65 μs</td>
    <td style="white-space: nowrap; text-align: right">±1249.33%</td>
    <td style="white-space: nowrap; text-align: right">0.50 μs</td>
    <td style="white-space: nowrap; text-align: right">3.74 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">283.09 K</td>
    <td style="white-space: nowrap; text-align: right">3.53 μs</td>
    <td style="white-space: nowrap; text-align: right">±93.99%</td>
    <td style="white-space: nowrap; text-align: right">3.09 μs</td>
    <td style="white-space: nowrap; text-align: right">21.52 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">List.replace_at/3</td>
    <td style="white-space: nowrap; text-align: right">5.70 K</td>
    <td style="white-space: nowrap; text-align: right">175.44 μs</td>
    <td style="white-space: nowrap; text-align: right">±95.29%</td>
    <td style="white-space: nowrap; text-align: right">122.07 μs</td>
    <td style="white-space: nowrap; text-align: right">730.54 μs</td>
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
    <td style="white-space: nowrap;text-align: right">538.24 K</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">377.42 K</td>
    <td style="white-space: nowrap; text-align: right">1.43x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">283.09 K</td>
    <td style="white-space: nowrap; text-align: right">1.9x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">List.replace_at/3</td>
    <td style="white-space: nowrap; text-align: right">5.70 K</td>
    <td style="white-space: nowrap; text-align: right">94.43x</td>
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
    <td style="white-space: nowrap; text-align: right">326.88 K</td>
    <td style="white-space: nowrap; text-align: right">3.06 μs</td>
    <td style="white-space: nowrap; text-align: right">±89.59%</td>
    <td style="white-space: nowrap; text-align: right">2.50 μs</td>
    <td style="white-space: nowrap; text-align: right">16.97 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">324.73 K</td>
    <td style="white-space: nowrap; text-align: right">3.08 μs</td>
    <td style="white-space: nowrap; text-align: right">±100.97%</td>
    <td style="white-space: nowrap; text-align: right">2.49 μs</td>
    <td style="white-space: nowrap; text-align: right">19.04 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">276.76 K</td>
    <td style="white-space: nowrap; text-align: right">3.61 μs</td>
    <td style="white-space: nowrap; text-align: right">±72.76%</td>
    <td style="white-space: nowrap; text-align: right">3.27 μs</td>
    <td style="white-space: nowrap; text-align: right">20.27 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">List.replace_at/3</td>
    <td style="white-space: nowrap; text-align: right">2.88 K</td>
    <td style="white-space: nowrap; text-align: right">346.84 μs</td>
    <td style="white-space: nowrap; text-align: right">±81.35%</td>
    <td style="white-space: nowrap; text-align: right">248.34 μs</td>
    <td style="white-space: nowrap; text-align: right">1101.52 μs</td>
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
    <td style="white-space: nowrap;text-align: right">326.88 K</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">324.73 K</td>
    <td style="white-space: nowrap; text-align: right">1.01x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">276.76 K</td>
    <td style="white-space: nowrap; text-align: right">1.18x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">List.replace_at/3</td>
    <td style="white-space: nowrap; text-align: right">2.88 K</td>
    <td style="white-space: nowrap; text-align: right">113.38x</td>
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
    <td style="white-space: nowrap">Arrays.replace/3 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">298.80 K</td>
    <td style="white-space: nowrap; text-align: right">3.35 μs</td>
    <td style="white-space: nowrap; text-align: right">±72.29%</td>
    <td style="white-space: nowrap; text-align: right">3.13 μs</td>
    <td style="white-space: nowrap; text-align: right">21.65 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">271.42 K</td>
    <td style="white-space: nowrap; text-align: right">3.68 μs</td>
    <td style="white-space: nowrap; text-align: right">±43.32%</td>
    <td style="white-space: nowrap; text-align: right">3.52 μs</td>
    <td style="white-space: nowrap; text-align: right">13.27 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">256.73 K</td>
    <td style="white-space: nowrap; text-align: right">3.90 μs</td>
    <td style="white-space: nowrap; text-align: right">±69.68%</td>
    <td style="white-space: nowrap; text-align: right">3.55 μs</td>
    <td style="white-space: nowrap; text-align: right">20.18 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">List.replace_at/3</td>
    <td style="white-space: nowrap; text-align: right">1.31 K</td>
    <td style="white-space: nowrap; text-align: right">765.18 μs</td>
    <td style="white-space: nowrap; text-align: right">±109.95%</td>
    <td style="white-space: nowrap; text-align: right">549.33 μs</td>
    <td style="white-space: nowrap; text-align: right">4071.78 μs</td>
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
    <td style="white-space: nowrap;text-align: right">298.80 K</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">271.42 K</td>
    <td style="white-space: nowrap; text-align: right">1.1x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">256.73 K</td>
    <td style="white-space: nowrap; text-align: right">1.16x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">List.replace_at/3</td>
    <td style="white-space: nowrap; text-align: right">1.31 K</td>
    <td style="white-space: nowrap; text-align: right">228.64x</td>
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
    <td style="white-space: nowrap; text-align: right">247.44 K</td>
    <td style="white-space: nowrap; text-align: right">4.04 μs</td>
    <td style="white-space: nowrap; text-align: right">±32.53%</td>
    <td style="white-space: nowrap; text-align: right">3.72 μs</td>
    <td style="white-space: nowrap; text-align: right">14.02 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">239.98 K</td>
    <td style="white-space: nowrap; text-align: right">4.17 μs</td>
    <td style="white-space: nowrap; text-align: right">±22.12%</td>
    <td style="white-space: nowrap; text-align: right">3.95 μs</td>
    <td style="white-space: nowrap; text-align: right">10.30 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">227.99 K</td>
    <td style="white-space: nowrap; text-align: right">4.39 μs</td>
    <td style="white-space: nowrap; text-align: right">±62.36%</td>
    <td style="white-space: nowrap; text-align: right">3.96 μs</td>
    <td style="white-space: nowrap; text-align: right">20.44 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">List.replace_at/3</td>
    <td style="white-space: nowrap; text-align: right">0.55 K</td>
    <td style="white-space: nowrap; text-align: right">1820.93 μs</td>
    <td style="white-space: nowrap; text-align: right">±99.95%</td>
    <td style="white-space: nowrap; text-align: right">1059.34 μs</td>
    <td style="white-space: nowrap; text-align: right">5799.82 μs</td>
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
    <td style="white-space: nowrap;text-align: right">247.44 K</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">239.98 K</td>
    <td style="white-space: nowrap; text-align: right">1.03x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">227.99 K</td>
    <td style="white-space: nowrap; text-align: right">1.09x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">List.replace_at/3</td>
    <td style="white-space: nowrap; text-align: right">0.55 K</td>
    <td style="white-space: nowrap; text-align: right">450.57x</td>
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
    <td style="white-space: nowrap">Arrays.replace/3 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">225.38 K</td>
    <td style="white-space: nowrap; text-align: right">4.44 μs</td>
    <td style="white-space: nowrap; text-align: right">±28.69%</td>
    <td style="white-space: nowrap; text-align: right">3.85 μs</td>
    <td style="white-space: nowrap; text-align: right">7.70 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">216.43 K</td>
    <td style="white-space: nowrap; text-align: right">4.62 μs</td>
    <td style="white-space: nowrap; text-align: right">±67.82%</td>
    <td style="white-space: nowrap; text-align: right">4.11 μs</td>
    <td style="white-space: nowrap; text-align: right">22.27 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">202.65 K</td>
    <td style="white-space: nowrap; text-align: right">4.93 μs</td>
    <td style="white-space: nowrap; text-align: right">±64.45%</td>
    <td style="white-space: nowrap; text-align: right">3.81 μs</td>
    <td style="white-space: nowrap; text-align: right">16.43 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">List.replace_at/3</td>
    <td style="white-space: nowrap; text-align: right">0.192 K</td>
    <td style="white-space: nowrap; text-align: right">5216.86 μs</td>
    <td style="white-space: nowrap; text-align: right">±99.11%</td>
    <td style="white-space: nowrap; text-align: right">4062.08 μs</td>
    <td style="white-space: nowrap; text-align: right">28913.46 μs</td>
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
    <td style="white-space: nowrap;text-align: right">225.38 K</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">216.43 K</td>
    <td style="white-space: nowrap; text-align: right">1.04x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">202.65 K</td>
    <td style="white-space: nowrap; text-align: right">1.11x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">List.replace_at/3</td>
    <td style="white-space: nowrap; text-align: right">0.192 K</td>
    <td style="white-space: nowrap; text-align: right">1175.79x</td>
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
    <td style="white-space: nowrap">Arrays.replace/3 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">227.28 K</td>
    <td style="white-space: nowrap; text-align: right">4.40 μs</td>
    <td style="white-space: nowrap; text-align: right">±14.58%</td>
    <td style="white-space: nowrap; text-align: right">4.21 μs</td>
    <td style="white-space: nowrap; text-align: right">5.57 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">222.05 K</td>
    <td style="white-space: nowrap; text-align: right">4.50 μs</td>
    <td style="white-space: nowrap; text-align: right">±15.56%</td>
    <td style="white-space: nowrap; text-align: right">4.44 μs</td>
    <td style="white-space: nowrap; text-align: right">5.52 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">202.32 K</td>
    <td style="white-space: nowrap; text-align: right">4.94 μs</td>
    <td style="white-space: nowrap; text-align: right">±69.52%</td>
    <td style="white-space: nowrap; text-align: right">4.32 μs</td>
    <td style="white-space: nowrap; text-align: right">22.52 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">List.replace_at/3</td>
    <td style="white-space: nowrap; text-align: right">0.0859 K</td>
    <td style="white-space: nowrap; text-align: right">11643.82 μs</td>
    <td style="white-space: nowrap; text-align: right">±144.25%</td>
    <td style="white-space: nowrap; text-align: right">3546.55 μs</td>
    <td style="white-space: nowrap; text-align: right">67028.41 μs</td>
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
    <td style="white-space: nowrap;text-align: right">227.28 K</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">222.05 K</td>
    <td style="white-space: nowrap; text-align: right">1.02x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.replace/3 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">202.32 K</td>
    <td style="white-space: nowrap; text-align: right">1.12x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">List.replace_at/3</td>
    <td style="white-space: nowrap; text-align: right">0.0859 K</td>
    <td style="white-space: nowrap; text-align: right">2646.43x</td>
  </tr>

</table>



<hr/>

