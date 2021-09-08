
# Benchmark

Compares random element access (for reading).

For arrays, `Arrays.get(myarray, index)` function can be used (myarray[index] will behave similarly as it uses `get` internally.)

For lists, `Enum.fetch(list, index)` is used.


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
    <td style="white-space: nowrap">Enum.fetch/2 (list)</td>
    <td style="white-space: nowrap; text-align: right">6.05 M</td>
    <td style="white-space: nowrap; text-align: right">165.34 ns</td>
    <td style="white-space: nowrap; text-align: right">±683.26%</td>
    <td style="white-space: nowrap; text-align: right">128 ns</td>
    <td style="white-space: nowrap; text-align: right">260 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">4.60 M</td>
    <td style="white-space: nowrap; text-align: right">217.27 ns</td>
    <td style="white-space: nowrap; text-align: right">±587.10%</td>
    <td style="white-space: nowrap; text-align: right">169 ns</td>
    <td style="white-space: nowrap; text-align: right">611.48 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">4.36 M</td>
    <td style="white-space: nowrap; text-align: right">229.18 ns</td>
    <td style="white-space: nowrap; text-align: right">±498.14%</td>
    <td style="white-space: nowrap; text-align: right">171 ns</td>
    <td style="white-space: nowrap; text-align: right">556 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">1.20 M</td>
    <td style="white-space: nowrap; text-align: right">831.43 ns</td>
    <td style="white-space: nowrap; text-align: right">±433.27%</td>
    <td style="white-space: nowrap; text-align: right">630 ns</td>
    <td style="white-space: nowrap; text-align: right">2255.10 ns</td>
  </tr>

</table>


Comparison

<table style="width: 1%">
  <tr>
    <th>Name</th>
    <th style="text-align: right">IPS</th>
    <th style="text-align: right">Slower</th>
  <tr>
    <td style="white-space: nowrap">Enum.fetch/2 (list)</td>
    <td style="white-space: nowrap;text-align: right">6.05 M</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">4.60 M</td>
    <td style="white-space: nowrap; text-align: right">1.31x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">4.36 M</td>
    <td style="white-space: nowrap; text-align: right">1.39x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">1.20 M</td>
    <td style="white-space: nowrap; text-align: right">5.03x</td>
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
    <td style="white-space: nowrap">Enum.fetch/2 (list)</td>
    <td style="white-space: nowrap; text-align: right">4.58 M</td>
    <td style="white-space: nowrap; text-align: right">218.28 ns</td>
    <td style="white-space: nowrap; text-align: right">±676.35%</td>
    <td style="white-space: nowrap; text-align: right">177 ns</td>
    <td style="white-space: nowrap; text-align: right">477 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">4.38 M</td>
    <td style="white-space: nowrap; text-align: right">228.21 ns</td>
    <td style="white-space: nowrap; text-align: right">±615.59%</td>
    <td style="white-space: nowrap; text-align: right">169 ns</td>
    <td style="white-space: nowrap; text-align: right">701.28 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">4.14 M</td>
    <td style="white-space: nowrap; text-align: right">241.29 ns</td>
    <td style="white-space: nowrap; text-align: right">±622.37%</td>
    <td style="white-space: nowrap; text-align: right">174 ns</td>
    <td style="white-space: nowrap; text-align: right">572 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">1.28 M</td>
    <td style="white-space: nowrap; text-align: right">783.37 ns</td>
    <td style="white-space: nowrap; text-align: right">±343.07%</td>
    <td style="white-space: nowrap; text-align: right">621 ns</td>
    <td style="white-space: nowrap; text-align: right">2075.08 ns</td>
  </tr>

</table>


Comparison

<table style="width: 1%">
  <tr>
    <th>Name</th>
    <th style="text-align: right">IPS</th>
    <th style="text-align: right">Slower</th>
  <tr>
    <td style="white-space: nowrap">Enum.fetch/2 (list)</td>
    <td style="white-space: nowrap;text-align: right">4.58 M</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">4.38 M</td>
    <td style="white-space: nowrap; text-align: right">1.05x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">4.14 M</td>
    <td style="white-space: nowrap; text-align: right">1.11x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">1.28 M</td>
    <td style="white-space: nowrap; text-align: right">3.59x</td>
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
    <td style="white-space: nowrap">Arrays.get/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">4.35 M</td>
    <td style="white-space: nowrap; text-align: right">229.73 ns</td>
    <td style="white-space: nowrap; text-align: right">±688.23%</td>
    <td style="white-space: nowrap; text-align: right">181 ns</td>
    <td style="white-space: nowrap; text-align: right">363.16 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">4.29 M</td>
    <td style="white-space: nowrap; text-align: right">233.33 ns</td>
    <td style="white-space: nowrap; text-align: right">±548.67%</td>
    <td style="white-space: nowrap; text-align: right">173 ns</td>
    <td style="white-space: nowrap; text-align: right">461.12 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Enum.fetch/2 (list)</td>
    <td style="white-space: nowrap; text-align: right">3.13 M</td>
    <td style="white-space: nowrap; text-align: right">319.96 ns</td>
    <td style="white-space: nowrap; text-align: right">±483.59%</td>
    <td style="white-space: nowrap; text-align: right">269 ns</td>
    <td style="white-space: nowrap; text-align: right">663.30 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">1.21 M</td>
    <td style="white-space: nowrap; text-align: right">826.60 ns</td>
    <td style="white-space: nowrap; text-align: right">±350.60%</td>
    <td style="white-space: nowrap; text-align: right">640 ns</td>
    <td style="white-space: nowrap; text-align: right">2752.51 ns</td>
  </tr>

</table>


Comparison

<table style="width: 1%">
  <tr>
    <th>Name</th>
    <th style="text-align: right">IPS</th>
    <th style="text-align: right">Slower</th>
  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (ErlangArray)</td>
    <td style="white-space: nowrap;text-align: right">4.35 M</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">4.29 M</td>
    <td style="white-space: nowrap; text-align: right">1.02x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Enum.fetch/2 (list)</td>
    <td style="white-space: nowrap; text-align: right">3.13 M</td>
    <td style="white-space: nowrap; text-align: right">1.39x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">1.21 M</td>
    <td style="white-space: nowrap; text-align: right">3.6x</td>
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
    <td style="white-space: nowrap">Arrays.get/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">4.09 M</td>
    <td style="white-space: nowrap; text-align: right">244.79 ns</td>
    <td style="white-space: nowrap; text-align: right">±683.04%</td>
    <td style="white-space: nowrap; text-align: right">182 ns</td>
    <td style="white-space: nowrap; text-align: right">628.88 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">4.00 M</td>
    <td style="white-space: nowrap; text-align: right">250.04 ns</td>
    <td style="white-space: nowrap; text-align: right">±496.67%</td>
    <td style="white-space: nowrap; text-align: right">193 ns</td>
    <td style="white-space: nowrap; text-align: right">382 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Enum.fetch/2 (list)</td>
    <td style="white-space: nowrap; text-align: right">2.04 M</td>
    <td style="white-space: nowrap; text-align: right">489.70 ns</td>
    <td style="white-space: nowrap; text-align: right">±233.14%</td>
    <td style="white-space: nowrap; text-align: right">434 ns</td>
    <td style="white-space: nowrap; text-align: right">985.19 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">1.18 M</td>
    <td style="white-space: nowrap; text-align: right">846.66 ns</td>
    <td style="white-space: nowrap; text-align: right">±348.42%</td>
    <td style="white-space: nowrap; text-align: right">656 ns</td>
    <td style="white-space: nowrap; text-align: right">2778.36 ns</td>
  </tr>

</table>


Comparison

<table style="width: 1%">
  <tr>
    <th>Name</th>
    <th style="text-align: right">IPS</th>
    <th style="text-align: right">Slower</th>
  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (MapArray)</td>
    <td style="white-space: nowrap;text-align: right">4.09 M</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">4.00 M</td>
    <td style="white-space: nowrap; text-align: right">1.02x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Enum.fetch/2 (list)</td>
    <td style="white-space: nowrap; text-align: right">2.04 M</td>
    <td style="white-space: nowrap; text-align: right">2.0x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">1.18 M</td>
    <td style="white-space: nowrap; text-align: right">3.46x</td>
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
    <td style="white-space: nowrap">Arrays.get/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">3.93 M</td>
    <td style="white-space: nowrap; text-align: right">254.40 ns</td>
    <td style="white-space: nowrap; text-align: right">±505.25%</td>
    <td style="white-space: nowrap; text-align: right">193 ns</td>
    <td style="white-space: nowrap; text-align: right">393.04 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">3.76 M</td>
    <td style="white-space: nowrap; text-align: right">266.31 ns</td>
    <td style="white-space: nowrap; text-align: right">±679.47%</td>
    <td style="white-space: nowrap; text-align: right">192 ns</td>
    <td style="white-space: nowrap; text-align: right">669 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Enum.fetch/2 (list)</td>
    <td style="white-space: nowrap; text-align: right">1.16 M</td>
    <td style="white-space: nowrap; text-align: right">859.70 ns</td>
    <td style="white-space: nowrap; text-align: right">±184.38%</td>
    <td style="white-space: nowrap; text-align: right">775 ns</td>
    <td style="white-space: nowrap; text-align: right">1698.30 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">1.13 M</td>
    <td style="white-space: nowrap; text-align: right">881.85 ns</td>
    <td style="white-space: nowrap; text-align: right">±334.53%</td>
    <td style="white-space: nowrap; text-align: right">679 ns</td>
    <td style="white-space: nowrap; text-align: right">2158.28 ns</td>
  </tr>

</table>


Comparison

<table style="width: 1%">
  <tr>
    <th>Name</th>
    <th style="text-align: right">IPS</th>
    <th style="text-align: right">Slower</th>
  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (ErlangArray)</td>
    <td style="white-space: nowrap;text-align: right">3.93 M</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">3.76 M</td>
    <td style="white-space: nowrap; text-align: right">1.05x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Enum.fetch/2 (list)</td>
    <td style="white-space: nowrap; text-align: right">1.16 M</td>
    <td style="white-space: nowrap; text-align: right">3.38x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">1.13 M</td>
    <td style="white-space: nowrap; text-align: right">3.47x</td>
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
    <td style="white-space: nowrap">Arrays.get/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">3.95 M</td>
    <td style="white-space: nowrap; text-align: right">252.97 ns</td>
    <td style="white-space: nowrap; text-align: right">±529.26%</td>
    <td style="white-space: nowrap; text-align: right">196 ns</td>
    <td style="white-space: nowrap; text-align: right">377.07 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">3.46 M</td>
    <td style="white-space: nowrap; text-align: right">288.76 ns</td>
    <td style="white-space: nowrap; text-align: right">±825.20%</td>
    <td style="white-space: nowrap; text-align: right">206 ns</td>
    <td style="white-space: nowrap; text-align: right">654.76 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">1.14 M</td>
    <td style="white-space: nowrap; text-align: right">876.73 ns</td>
    <td style="white-space: nowrap; text-align: right">±329.32%</td>
    <td style="white-space: nowrap; text-align: right">673 ns</td>
    <td style="white-space: nowrap; text-align: right">2880.84 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Enum.fetch/2 (list)</td>
    <td style="white-space: nowrap; text-align: right">0.62 M</td>
    <td style="white-space: nowrap; text-align: right">1614.59 ns</td>
    <td style="white-space: nowrap; text-align: right">±114.70%</td>
    <td style="white-space: nowrap; text-align: right">1501 ns</td>
    <td style="white-space: nowrap; text-align: right">3275.66 ns</td>
  </tr>

</table>


Comparison

<table style="width: 1%">
  <tr>
    <th>Name</th>
    <th style="text-align: right">IPS</th>
    <th style="text-align: right">Slower</th>
  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (ErlangArray)</td>
    <td style="white-space: nowrap;text-align: right">3.95 M</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">3.46 M</td>
    <td style="white-space: nowrap; text-align: right">1.14x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">1.14 M</td>
    <td style="white-space: nowrap; text-align: right">3.47x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Enum.fetch/2 (list)</td>
    <td style="white-space: nowrap; text-align: right">0.62 M</td>
    <td style="white-space: nowrap; text-align: right">6.38x</td>
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
    <td style="white-space: nowrap">Arrays.get/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">3.92 M</td>
    <td style="white-space: nowrap; text-align: right">255.25 ns</td>
    <td style="white-space: nowrap; text-align: right">±442.78%</td>
    <td style="white-space: nowrap; text-align: right">208 ns</td>
    <td style="white-space: nowrap; text-align: right">410.86 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">3.76 M</td>
    <td style="white-space: nowrap; text-align: right">265.61 ns</td>
    <td style="white-space: nowrap; text-align: right">±429.14%</td>
    <td style="white-space: nowrap; text-align: right">219 ns</td>
    <td style="white-space: nowrap; text-align: right">532.79 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">1.19 M</td>
    <td style="white-space: nowrap; text-align: right">838.07 ns</td>
    <td style="white-space: nowrap; text-align: right">±327.05%</td>
    <td style="white-space: nowrap; text-align: right">671 ns</td>
    <td style="white-space: nowrap; text-align: right">2802.55 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Enum.fetch/2 (list)</td>
    <td style="white-space: nowrap; text-align: right">0.32 M</td>
    <td style="white-space: nowrap; text-align: right">3121.34 ns</td>
    <td style="white-space: nowrap; text-align: right">±83.90%</td>
    <td style="white-space: nowrap; text-align: right">2967 ns</td>
    <td style="white-space: nowrap; text-align: right">8414.32 ns</td>
  </tr>

</table>


Comparison

<table style="width: 1%">
  <tr>
    <th>Name</th>
    <th style="text-align: right">IPS</th>
    <th style="text-align: right">Slower</th>
  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (ErlangArray)</td>
    <td style="white-space: nowrap;text-align: right">3.92 M</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">3.76 M</td>
    <td style="white-space: nowrap; text-align: right">1.04x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">1.19 M</td>
    <td style="white-space: nowrap; text-align: right">3.28x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Enum.fetch/2 (list)</td>
    <td style="white-space: nowrap; text-align: right">0.32 M</td>
    <td style="white-space: nowrap; text-align: right">12.23x</td>
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
    <td style="white-space: nowrap">Arrays.get/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">3.64 M</td>
    <td style="white-space: nowrap; text-align: right">275.06 ns</td>
    <td style="white-space: nowrap; text-align: right">±621.06%</td>
    <td style="white-space: nowrap; text-align: right">215 ns</td>
    <td style="white-space: nowrap; text-align: right">468.25 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">3.45 M</td>
    <td style="white-space: nowrap; text-align: right">289.71 ns</td>
    <td style="white-space: nowrap; text-align: right">±603.20%</td>
    <td style="white-space: nowrap; text-align: right">228 ns</td>
    <td style="white-space: nowrap; text-align: right">513 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">1.20 M</td>
    <td style="white-space: nowrap; text-align: right">831.00 ns</td>
    <td style="white-space: nowrap; text-align: right">±268.53%</td>
    <td style="white-space: nowrap; text-align: right">697 ns</td>
    <td style="white-space: nowrap; text-align: right">1457.04 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Enum.fetch/2 (list)</td>
    <td style="white-space: nowrap; text-align: right">0.164 M</td>
    <td style="white-space: nowrap; text-align: right">6113.78 ns</td>
    <td style="white-space: nowrap; text-align: right">±75.90%</td>
    <td style="white-space: nowrap; text-align: right">5759 ns</td>
    <td style="white-space: nowrap; text-align: right">26170.70 ns</td>
  </tr>

</table>


Comparison

<table style="width: 1%">
  <tr>
    <th>Name</th>
    <th style="text-align: right">IPS</th>
    <th style="text-align: right">Slower</th>
  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (MapArray)</td>
    <td style="white-space: nowrap;text-align: right">3.64 M</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">3.45 M</td>
    <td style="white-space: nowrap; text-align: right">1.05x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">1.20 M</td>
    <td style="white-space: nowrap; text-align: right">3.02x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Enum.fetch/2 (list)</td>
    <td style="white-space: nowrap; text-align: right">0.164 M</td>
    <td style="white-space: nowrap; text-align: right">22.23x</td>
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
    <td style="white-space: nowrap">Arrays.get/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">3.81 M</td>
    <td style="white-space: nowrap; text-align: right">262.37 ns</td>
    <td style="white-space: nowrap; text-align: right">±228.60%</td>
    <td style="white-space: nowrap; text-align: right">231 ns</td>
    <td style="white-space: nowrap; text-align: right">465.05 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">3.72 M</td>
    <td style="white-space: nowrap; text-align: right">268.64 ns</td>
    <td style="white-space: nowrap; text-align: right">±218.26%</td>
    <td style="white-space: nowrap; text-align: right">240 ns</td>
    <td style="white-space: nowrap; text-align: right">432.86 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">1.14 M</td>
    <td style="white-space: nowrap; text-align: right">877.89 ns</td>
    <td style="white-space: nowrap; text-align: right">±316.29%</td>
    <td style="white-space: nowrap; text-align: right">717 ns</td>
    <td style="white-space: nowrap; text-align: right">1503.30 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Enum.fetch/2 (list)</td>
    <td style="white-space: nowrap; text-align: right">0.0832 M</td>
    <td style="white-space: nowrap; text-align: right">12023.31 ns</td>
    <td style="white-space: nowrap; text-align: right">±70.26%</td>
    <td style="white-space: nowrap; text-align: right">11455 ns</td>
    <td style="white-space: nowrap; text-align: right">42274.86 ns</td>
  </tr>

</table>


Comparison

<table style="width: 1%">
  <tr>
    <th>Name</th>
    <th style="text-align: right">IPS</th>
    <th style="text-align: right">Slower</th>
  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (MapArray)</td>
    <td style="white-space: nowrap;text-align: right">3.81 M</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">3.72 M</td>
    <td style="white-space: nowrap; text-align: right">1.02x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">1.14 M</td>
    <td style="white-space: nowrap; text-align: right">3.35x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Enum.fetch/2 (list)</td>
    <td style="white-space: nowrap; text-align: right">0.0832 M</td>
    <td style="white-space: nowrap; text-align: right">45.83x</td>
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
    <td style="white-space: nowrap">Arrays.get/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">3.76 M</td>
    <td style="white-space: nowrap; text-align: right">266.28 ns</td>
    <td style="white-space: nowrap; text-align: right">±185.26%</td>
    <td style="white-space: nowrap; text-align: right">236 ns</td>
    <td style="white-space: nowrap; text-align: right">519.20 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">2.89 M</td>
    <td style="white-space: nowrap; text-align: right">346.46 ns</td>
    <td style="white-space: nowrap; text-align: right">±227.78%</td>
    <td style="white-space: nowrap; text-align: right">261 ns</td>
    <td style="white-space: nowrap; text-align: right">1021.08 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">1.14 M</td>
    <td style="white-space: nowrap; text-align: right">880.74 ns</td>
    <td style="white-space: nowrap; text-align: right">±255.86%</td>
    <td style="white-space: nowrap; text-align: right">732 ns</td>
    <td style="white-space: nowrap; text-align: right">1872.40 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Enum.fetch/2 (list)</td>
    <td style="white-space: nowrap; text-align: right">0.0421 M</td>
    <td style="white-space: nowrap; text-align: right">23755.10 ns</td>
    <td style="white-space: nowrap; text-align: right">±65.19%</td>
    <td style="white-space: nowrap; text-align: right">23426 ns</td>
    <td style="white-space: nowrap; text-align: right">77065.16 ns</td>
  </tr>

</table>


Comparison

<table style="width: 1%">
  <tr>
    <th>Name</th>
    <th style="text-align: right">IPS</th>
    <th style="text-align: right">Slower</th>
  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (MapArray)</td>
    <td style="white-space: nowrap;text-align: right">3.76 M</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">2.89 M</td>
    <td style="white-space: nowrap; text-align: right">1.3x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">1.14 M</td>
    <td style="white-space: nowrap; text-align: right">3.31x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Enum.fetch/2 (list)</td>
    <td style="white-space: nowrap; text-align: right">0.0421 M</td>
    <td style="white-space: nowrap; text-align: right">89.21x</td>
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
    <td style="white-space: nowrap">Arrays.get/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">1031.34 K</td>
    <td style="white-space: nowrap; text-align: right">0.97 μs</td>
    <td style="white-space: nowrap; text-align: right">±317.08%</td>
    <td style="white-space: nowrap; text-align: right">0.76 μs</td>
    <td style="white-space: nowrap; text-align: right">2.40 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">637.95 K</td>
    <td style="white-space: nowrap; text-align: right">1.57 μs</td>
    <td style="white-space: nowrap; text-align: right">±1590.84%</td>
    <td style="white-space: nowrap; text-align: right">0.27 μs</td>
    <td style="white-space: nowrap; text-align: right">1.88 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">637.74 K</td>
    <td style="white-space: nowrap; text-align: right">1.57 μs</td>
    <td style="white-space: nowrap; text-align: right">±1633.33%</td>
    <td style="white-space: nowrap; text-align: right">0.28 μs</td>
    <td style="white-space: nowrap; text-align: right">1.31 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Enum.fetch/2 (list)</td>
    <td style="white-space: nowrap; text-align: right">21.20 K</td>
    <td style="white-space: nowrap; text-align: right">47.18 μs</td>
    <td style="white-space: nowrap; text-align: right">±64.50%</td>
    <td style="white-space: nowrap; text-align: right">45.52 μs</td>
    <td style="white-space: nowrap; text-align: right">152.74 μs</td>
  </tr>

</table>


Comparison

<table style="width: 1%">
  <tr>
    <th>Name</th>
    <th style="text-align: right">IPS</th>
    <th style="text-align: right">Slower</th>
  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (RRBVector)</td>
    <td style="white-space: nowrap;text-align: right">1031.34 K</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">637.95 K</td>
    <td style="white-space: nowrap; text-align: right">1.62x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">637.74 K</td>
    <td style="white-space: nowrap; text-align: right">1.62x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Enum.fetch/2 (list)</td>
    <td style="white-space: nowrap; text-align: right">21.20 K</td>
    <td style="white-space: nowrap; text-align: right">48.66x</td>
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
    <td style="white-space: nowrap">Arrays.get/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">1033.40 K</td>
    <td style="white-space: nowrap; text-align: right">0.97 μs</td>
    <td style="white-space: nowrap; text-align: right">±244.61%</td>
    <td style="white-space: nowrap; text-align: right">0.77 μs</td>
    <td style="white-space: nowrap; text-align: right">3.00 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">559.43 K</td>
    <td style="white-space: nowrap; text-align: right">1.79 μs</td>
    <td style="white-space: nowrap; text-align: right">±98.81%</td>
    <td style="white-space: nowrap; text-align: right">1.58 μs</td>
    <td style="white-space: nowrap; text-align: right">14.97 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">511.31 K</td>
    <td style="white-space: nowrap; text-align: right">1.96 μs</td>
    <td style="white-space: nowrap; text-align: right">±113.90%</td>
    <td style="white-space: nowrap; text-align: right">1.59 μs</td>
    <td style="white-space: nowrap; text-align: right">14.92 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Enum.fetch/2 (list)</td>
    <td style="white-space: nowrap; text-align: right">10.45 K</td>
    <td style="white-space: nowrap; text-align: right">95.66 μs</td>
    <td style="white-space: nowrap; text-align: right">±63.53%</td>
    <td style="white-space: nowrap; text-align: right">93.37 μs</td>
    <td style="white-space: nowrap; text-align: right">257.46 μs</td>
  </tr>

</table>


Comparison

<table style="width: 1%">
  <tr>
    <th>Name</th>
    <th style="text-align: right">IPS</th>
    <th style="text-align: right">Slower</th>
  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (RRBVector)</td>
    <td style="white-space: nowrap;text-align: right">1033.40 K</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">559.43 K</td>
    <td style="white-space: nowrap; text-align: right">1.85x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">511.31 K</td>
    <td style="white-space: nowrap; text-align: right">2.02x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Enum.fetch/2 (list)</td>
    <td style="white-space: nowrap; text-align: right">10.45 K</td>
    <td style="white-space: nowrap; text-align: right">98.86x</td>
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
    <td style="white-space: nowrap">Arrays.get/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">989.34 K</td>
    <td style="white-space: nowrap; text-align: right">1.01 μs</td>
    <td style="white-space: nowrap; text-align: right">±234.44%</td>
    <td style="white-space: nowrap; text-align: right">0.81 μs</td>
    <td style="white-space: nowrap; text-align: right">3.47 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">348.85 K</td>
    <td style="white-space: nowrap; text-align: right">2.87 μs</td>
    <td style="white-space: nowrap; text-align: right">±49.50%</td>
    <td style="white-space: nowrap; text-align: right">2.69 μs</td>
    <td style="white-space: nowrap; text-align: right">9.79 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">323.34 K</td>
    <td style="white-space: nowrap; text-align: right">3.09 μs</td>
    <td style="white-space: nowrap; text-align: right">±54.55%</td>
    <td style="white-space: nowrap; text-align: right">2.80 μs</td>
    <td style="white-space: nowrap; text-align: right">15.05 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Enum.fetch/2 (list)</td>
    <td style="white-space: nowrap; text-align: right">4.24 K</td>
    <td style="white-space: nowrap; text-align: right">235.79 μs</td>
    <td style="white-space: nowrap; text-align: right">±60.29%</td>
    <td style="white-space: nowrap; text-align: right">227.31 μs</td>
    <td style="white-space: nowrap; text-align: right">582.64 μs</td>
  </tr>

</table>


Comparison

<table style="width: 1%">
  <tr>
    <th>Name</th>
    <th style="text-align: right">IPS</th>
    <th style="text-align: right">Slower</th>
  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (RRBVector)</td>
    <td style="white-space: nowrap;text-align: right">989.34 K</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">348.85 K</td>
    <td style="white-space: nowrap; text-align: right">2.84x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">323.34 K</td>
    <td style="white-space: nowrap; text-align: right">3.06x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Enum.fetch/2 (list)</td>
    <td style="white-space: nowrap; text-align: right">4.24 K</td>
    <td style="white-space: nowrap; text-align: right">233.27x</td>
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
    <td style="white-space: nowrap">Arrays.get/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">988.66 K</td>
    <td style="white-space: nowrap; text-align: right">1.01 μs</td>
    <td style="white-space: nowrap; text-align: right">±225.04%</td>
    <td style="white-space: nowrap; text-align: right">0.82 μs</td>
    <td style="white-space: nowrap; text-align: right">3.05 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">339.71 K</td>
    <td style="white-space: nowrap; text-align: right">2.94 μs</td>
    <td style="white-space: nowrap; text-align: right">±42.21%</td>
    <td style="white-space: nowrap; text-align: right">2.80 μs</td>
    <td style="white-space: nowrap; text-align: right">14.56 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">275.24 K</td>
    <td style="white-space: nowrap; text-align: right">3.63 μs</td>
    <td style="white-space: nowrap; text-align: right">±66.18%</td>
    <td style="white-space: nowrap; text-align: right">3.04 μs</td>
    <td style="white-space: nowrap; text-align: right">23.78 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Enum.fetch/2 (list)</td>
    <td style="white-space: nowrap; text-align: right">1.83 K</td>
    <td style="white-space: nowrap; text-align: right">545.19 μs</td>
    <td style="white-space: nowrap; text-align: right">±49.54%</td>
    <td style="white-space: nowrap; text-align: right">538.52 μs</td>
    <td style="white-space: nowrap; text-align: right">1017.10 μs</td>
  </tr>

</table>


Comparison

<table style="width: 1%">
  <tr>
    <th>Name</th>
    <th style="text-align: right">IPS</th>
    <th style="text-align: right">Slower</th>
  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (RRBVector)</td>
    <td style="white-space: nowrap;text-align: right">988.66 K</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">339.71 K</td>
    <td style="white-space: nowrap; text-align: right">2.91x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">275.24 K</td>
    <td style="white-space: nowrap; text-align: right">3.59x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Enum.fetch/2 (list)</td>
    <td style="white-space: nowrap; text-align: right">1.83 K</td>
    <td style="white-space: nowrap; text-align: right">539.01x</td>
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
    <td style="white-space: nowrap">Arrays.get/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">871.00 K</td>
    <td style="white-space: nowrap; text-align: right">1.15 μs</td>
    <td style="white-space: nowrap; text-align: right">±226.95%</td>
    <td style="white-space: nowrap; text-align: right">0.87 μs</td>
    <td style="white-space: nowrap; text-align: right">4.69 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">356.69 K</td>
    <td style="white-space: nowrap; text-align: right">2.80 μs</td>
    <td style="white-space: nowrap; text-align: right">±17.72%</td>
    <td style="white-space: nowrap; text-align: right">2.66 μs</td>
    <td style="white-space: nowrap; text-align: right">4.09 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">306.22 K</td>
    <td style="white-space: nowrap; text-align: right">3.27 μs</td>
    <td style="white-space: nowrap; text-align: right">±81.63%</td>
    <td style="white-space: nowrap; text-align: right">2.73 μs</td>
    <td style="white-space: nowrap; text-align: right">20.29 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Enum.fetch/2 (list)</td>
    <td style="white-space: nowrap; text-align: right">0.98 K</td>
    <td style="white-space: nowrap; text-align: right">1015.59 μs</td>
    <td style="white-space: nowrap; text-align: right">±64.89%</td>
    <td style="white-space: nowrap; text-align: right">1139.35 μs</td>
    <td style="white-space: nowrap; text-align: right">2863.40 μs</td>
  </tr>

</table>


Comparison

<table style="width: 1%">
  <tr>
    <th>Name</th>
    <th style="text-align: right">IPS</th>
    <th style="text-align: right">Slower</th>
  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (RRBVector)</td>
    <td style="white-space: nowrap;text-align: right">871.00 K</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">356.69 K</td>
    <td style="white-space: nowrap; text-align: right">2.44x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">306.22 K</td>
    <td style="white-space: nowrap; text-align: right">2.84x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Enum.fetch/2 (list)</td>
    <td style="white-space: nowrap; text-align: right">0.98 K</td>
    <td style="white-space: nowrap; text-align: right">884.58x</td>
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
    <td style="white-space: nowrap">Arrays.get/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">842.96 K</td>
    <td style="white-space: nowrap; text-align: right">1.19 μs</td>
    <td style="white-space: nowrap; text-align: right">±216.82%</td>
    <td style="white-space: nowrap; text-align: right">0.92 μs</td>
    <td style="white-space: nowrap; text-align: right">4.75 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">294.62 K</td>
    <td style="white-space: nowrap; text-align: right">3.39 μs</td>
    <td style="white-space: nowrap; text-align: right">±14.95%</td>
    <td style="white-space: nowrap; text-align: right">3.30 μs</td>
    <td style="white-space: nowrap; text-align: right">4.87 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">243.18 K</td>
    <td style="white-space: nowrap; text-align: right">4.11 μs</td>
    <td style="white-space: nowrap; text-align: right">±102.04%</td>
    <td style="white-space: nowrap; text-align: right">3.16 μs</td>
    <td style="white-space: nowrap; text-align: right">22.77 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Enum.fetch/2 (list)</td>
    <td style="white-space: nowrap; text-align: right">0.49 K</td>
    <td style="white-space: nowrap; text-align: right">2056.96 μs</td>
    <td style="white-space: nowrap; text-align: right">±51.65%</td>
    <td style="white-space: nowrap; text-align: right">2195.10 μs</td>
    <td style="white-space: nowrap; text-align: right">3979.17 μs</td>
  </tr>

</table>


Comparison

<table style="width: 1%">
  <tr>
    <th>Name</th>
    <th style="text-align: right">IPS</th>
    <th style="text-align: right">Slower</th>
  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (RRBVector)</td>
    <td style="white-space: nowrap;text-align: right">842.96 K</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">294.62 K</td>
    <td style="white-space: nowrap; text-align: right">2.86x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">243.18 K</td>
    <td style="white-space: nowrap; text-align: right">3.47x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Enum.fetch/2 (list)</td>
    <td style="white-space: nowrap; text-align: right">0.49 K</td>
    <td style="white-space: nowrap; text-align: right">1733.93x</td>
  </tr>

</table>



<hr/>

