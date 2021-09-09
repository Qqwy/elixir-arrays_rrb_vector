
# Benchmark

Comparing `Arrays.append` with appending a value to a list.


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
    <td style="white-space: nowrap">[val | list] (list, backwards)</td>
    <td style="white-space: nowrap; text-align: right">18.03 M</td>
    <td style="white-space: nowrap; text-align: right">55.46 ns</td>
    <td style="white-space: nowrap; text-align: right">±1815.06%</td>
    <td style="white-space: nowrap; text-align: right">34 ns</td>
    <td style="white-space: nowrap; text-align: right">62 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">3.72 M</td>
    <td style="white-space: nowrap; text-align: right">269.13 ns</td>
    <td style="white-space: nowrap; text-align: right">±551.28%</td>
    <td style="white-space: nowrap; text-align: right">168 ns</td>
    <td style="white-space: nowrap; text-align: right">570.81 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">list ++ [val] (list)</td>
    <td style="white-space: nowrap; text-align: right">2.58 M</td>
    <td style="white-space: nowrap; text-align: right">388.12 ns</td>
    <td style="white-space: nowrap; text-align: right">±833.14%</td>
    <td style="white-space: nowrap; text-align: right">128 ns</td>
    <td style="white-space: nowrap; text-align: right">3953.60 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">1.17 M</td>
    <td style="white-space: nowrap; text-align: right">852.61 ns</td>
    <td style="white-space: nowrap; text-align: right">±300.72%</td>
    <td style="white-space: nowrap; text-align: right">641 ns</td>
    <td style="white-space: nowrap; text-align: right">1562.88 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">0.37 M</td>
    <td style="white-space: nowrap; text-align: right">2707.40 ns</td>
    <td style="white-space: nowrap; text-align: right">±145.37%</td>
    <td style="white-space: nowrap; text-align: right">2075 ns</td>
    <td style="white-space: nowrap; text-align: right">21904.52 ns</td>
  </tr>

</table>


Comparison

<table style="width: 1%">
  <tr>
    <th>Name</th>
    <th style="text-align: right">IPS</th>
    <th style="text-align: right">Slower</th>
  <tr>
    <td style="white-space: nowrap">[val | list] (list, backwards)</td>
    <td style="white-space: nowrap;text-align: right">18.03 M</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">3.72 M</td>
    <td style="white-space: nowrap; text-align: right">4.85x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">list ++ [val] (list)</td>
    <td style="white-space: nowrap; text-align: right">2.58 M</td>
    <td style="white-space: nowrap; text-align: right">7.0x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">1.17 M</td>
    <td style="white-space: nowrap; text-align: right">15.37x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">0.37 M</td>
    <td style="white-space: nowrap; text-align: right">48.82x</td>
  </tr>

</table>



Memory Usage

<table style="width: 1%">
  <tr>
    <th>Name</th>
    <th style="text-align: right">Memory</th>
<th style="text-align: right">Factor</th>
  </tr>
  <tr>
    <td style="white-space: nowrap">[val | list] (list, backwards)</td>
    <td style="white-space: nowrap">16 B</td>
<td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (ErlangArray)</td>
    <td style="white-space: nowrap">272 B</td>
    <td>17.0x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">list ++ [val] (list)</td>
    <td style="white-space: nowrap">528 B</td>
    <td>33.0x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (RRBVector)</td>
    <td style="white-space: nowrap">88 B</td>
    <td>5.5x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (MapArray)</td>
    <td style="white-space: nowrap">920 B</td>
    <td>57.5x</td>
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
    <td style="white-space: nowrap">[val | list] (list, backwards)</td>
    <td style="white-space: nowrap; text-align: right">20.37 M</td>
    <td style="white-space: nowrap; text-align: right">49.08 ns</td>
    <td style="white-space: nowrap; text-align: right">±936.97%</td>
    <td style="white-space: nowrap; text-align: right">36 ns</td>
    <td style="white-space: nowrap; text-align: right">72 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">4.54 M</td>
    <td style="white-space: nowrap; text-align: right">220.15 ns</td>
    <td style="white-space: nowrap; text-align: right">±446.12%</td>
    <td style="white-space: nowrap; text-align: right">162 ns</td>
    <td style="white-space: nowrap; text-align: right">498.54 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">4.03 M</td>
    <td style="white-space: nowrap; text-align: right">248.10 ns</td>
    <td style="white-space: nowrap; text-align: right">±548.95%</td>
    <td style="white-space: nowrap; text-align: right">165 ns</td>
    <td style="white-space: nowrap; text-align: right">395.91 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">list ++ [val] (list)</td>
    <td style="white-space: nowrap; text-align: right">1.81 M</td>
    <td style="white-space: nowrap; text-align: right">551.45 ns</td>
    <td style="white-space: nowrap; text-align: right">±517.57%</td>
    <td style="white-space: nowrap; text-align: right">217 ns</td>
    <td style="white-space: nowrap; text-align: right">12372.60 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.90 M</td>
    <td style="white-space: nowrap; text-align: right">1112.15 ns</td>
    <td style="white-space: nowrap; text-align: right">±194.38%</td>
    <td style="white-space: nowrap; text-align: right">896 ns</td>
    <td style="white-space: nowrap; text-align: right">5252.01 ns</td>
  </tr>

</table>


Comparison

<table style="width: 1%">
  <tr>
    <th>Name</th>
    <th style="text-align: right">IPS</th>
    <th style="text-align: right">Slower</th>
  <tr>
    <td style="white-space: nowrap">[val | list] (list, backwards)</td>
    <td style="white-space: nowrap;text-align: right">20.37 M</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">4.54 M</td>
    <td style="white-space: nowrap; text-align: right">4.49x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">4.03 M</td>
    <td style="white-space: nowrap; text-align: right">5.05x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">list ++ [val] (list)</td>
    <td style="white-space: nowrap; text-align: right">1.81 M</td>
    <td style="white-space: nowrap; text-align: right">11.23x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.90 M</td>
    <td style="white-space: nowrap; text-align: right">22.66x</td>
  </tr>

</table>



Memory Usage

<table style="width: 1%">
  <tr>
    <th>Name</th>
    <th style="text-align: right">Memory</th>
<th style="text-align: right">Factor</th>
  </tr>
  <tr>
    <td style="white-space: nowrap">[val | list] (list, backwards)</td>
    <td style="white-space: nowrap">16 B</td>
<td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (ErlangArray)</td>
    <td style="white-space: nowrap">272 B</td>
    <td>17.0x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (MapArray)</td>
    <td style="white-space: nowrap">224 B</td>
    <td>14.0x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">list ++ [val] (list)</td>
    <td style="white-space: nowrap">448 B</td>
    <td>28.0x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (RRBVector)</td>
    <td style="white-space: nowrap">88 B</td>
    <td>5.5x</td>
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
    <td style="white-space: nowrap">[val | list] (list, backwards)</td>
    <td style="white-space: nowrap; text-align: right">14.32 M</td>
    <td style="white-space: nowrap; text-align: right">69.85 ns</td>
    <td style="white-space: nowrap; text-align: right">±1257.32%</td>
    <td style="white-space: nowrap; text-align: right">41 ns</td>
    <td style="white-space: nowrap; text-align: right">80 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">3.02 M</td>
    <td style="white-space: nowrap; text-align: right">330.79 ns</td>
    <td style="white-space: nowrap; text-align: right">±439.52%</td>
    <td style="white-space: nowrap; text-align: right">204 ns</td>
    <td style="white-space: nowrap; text-align: right">3264.30 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">3.00 M</td>
    <td style="white-space: nowrap; text-align: right">332.81 ns</td>
    <td style="white-space: nowrap; text-align: right">±451.07%</td>
    <td style="white-space: nowrap; text-align: right">223 ns</td>
    <td style="white-space: nowrap; text-align: right">1744.15 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">list ++ [val] (list)</td>
    <td style="white-space: nowrap; text-align: right">0.98 M</td>
    <td style="white-space: nowrap; text-align: right">1017.03 ns</td>
    <td style="white-space: nowrap; text-align: right">±387.01%</td>
    <td style="white-space: nowrap; text-align: right">373 ns</td>
    <td style="white-space: nowrap; text-align: right">19946.75 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.95 M</td>
    <td style="white-space: nowrap; text-align: right">1056.68 ns</td>
    <td style="white-space: nowrap; text-align: right">±326.14%</td>
    <td style="white-space: nowrap; text-align: right">737 ns</td>
    <td style="white-space: nowrap; text-align: right">10128.68 ns</td>
  </tr>

</table>


Comparison

<table style="width: 1%">
  <tr>
    <th>Name</th>
    <th style="text-align: right">IPS</th>
    <th style="text-align: right">Slower</th>
  <tr>
    <td style="white-space: nowrap">[val | list] (list, backwards)</td>
    <td style="white-space: nowrap;text-align: right">14.32 M</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">3.02 M</td>
    <td style="white-space: nowrap; text-align: right">4.74x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">3.00 M</td>
    <td style="white-space: nowrap; text-align: right">4.76x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">list ++ [val] (list)</td>
    <td style="white-space: nowrap; text-align: right">0.98 M</td>
    <td style="white-space: nowrap; text-align: right">14.56x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.95 M</td>
    <td style="white-space: nowrap; text-align: right">15.13x</td>
  </tr>

</table>



Memory Usage

<table style="width: 1%">
  <tr>
    <th>Name</th>
    <th style="text-align: right">Memory</th>
<th style="text-align: right">Factor</th>
  </tr>
  <tr>
    <td style="white-space: nowrap">[val | list] (list, backwards)</td>
    <td style="white-space: nowrap">16 B</td>
<td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (MapArray)</td>
    <td style="white-space: nowrap">264 B</td>
    <td>16.5x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (ErlangArray)</td>
    <td style="white-space: nowrap">368 B</td>
    <td>23.0x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">list ++ [val] (list)</td>
    <td style="white-space: nowrap">560 B</td>
    <td>35.0x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (RRBVector)</td>
    <td style="white-space: nowrap">88 B</td>
    <td>5.5x</td>
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
    <td style="white-space: nowrap">[val | list] (list, backwards)</td>
    <td style="white-space: nowrap; text-align: right">12.52 M</td>
    <td style="white-space: nowrap; text-align: right">79.90 ns</td>
    <td style="white-space: nowrap; text-align: right">±961.08%</td>
    <td style="white-space: nowrap; text-align: right">47 ns</td>
    <td style="white-space: nowrap; text-align: right">97 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">3.49 M</td>
    <td style="white-space: nowrap; text-align: right">286.61 ns</td>
    <td style="white-space: nowrap; text-align: right">±291.47%</td>
    <td style="white-space: nowrap; text-align: right">239 ns</td>
    <td style="white-space: nowrap; text-align: right">460.68 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">3.30 M</td>
    <td style="white-space: nowrap; text-align: right">302.98 ns</td>
    <td style="white-space: nowrap; text-align: right">±290.61%</td>
    <td style="white-space: nowrap; text-align: right">245 ns</td>
    <td style="white-space: nowrap; text-align: right">438.70 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">list ++ [val] (list)</td>
    <td style="white-space: nowrap; text-align: right">0.56 M</td>
    <td style="white-space: nowrap; text-align: right">1790.46 ns</td>
    <td style="white-space: nowrap; text-align: right">±246.86%</td>
    <td style="white-space: nowrap; text-align: right">705 ns</td>
    <td style="white-space: nowrap; text-align: right">25348.65 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.51 M</td>
    <td style="white-space: nowrap; text-align: right">1959.35 ns</td>
    <td style="white-space: nowrap; text-align: right">±108.08%</td>
    <td style="white-space: nowrap; text-align: right">1677 ns</td>
    <td style="white-space: nowrap; text-align: right">16455.40 ns</td>
  </tr>

</table>


Comparison

<table style="width: 1%">
  <tr>
    <th>Name</th>
    <th style="text-align: right">IPS</th>
    <th style="text-align: right">Slower</th>
  <tr>
    <td style="white-space: nowrap">[val | list] (list, backwards)</td>
    <td style="white-space: nowrap;text-align: right">12.52 M</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">3.49 M</td>
    <td style="white-space: nowrap; text-align: right">3.59x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">3.30 M</td>
    <td style="white-space: nowrap; text-align: right">3.79x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">list ++ [val] (list)</td>
    <td style="white-space: nowrap; text-align: right">0.56 M</td>
    <td style="white-space: nowrap; text-align: right">22.41x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.51 M</td>
    <td style="white-space: nowrap; text-align: right">24.52x</td>
  </tr>

</table>



Memory Usage

<table style="width: 1%">
  <tr>
    <th>Name</th>
    <th style="text-align: right">Memory</th>
<th style="text-align: right">Factor</th>
  </tr>
  <tr>
    <td style="white-space: nowrap">[val | list] (list, backwards)</td>
    <td style="white-space: nowrap">16 B</td>
<td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (ErlangArray)</td>
    <td style="white-space: nowrap">368 B</td>
    <td>23.0x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (MapArray)</td>
    <td style="white-space: nowrap">328 B</td>
    <td>20.5x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">list ++ [val] (list)</td>
    <td style="white-space: nowrap">384 B</td>
    <td>24.0x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (RRBVector)</td>
    <td style="white-space: nowrap">88 B</td>
    <td>5.5x</td>
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
    <td style="white-space: nowrap">[val | list] (list, backwards)</td>
    <td style="white-space: nowrap; text-align: right">15.02 M</td>
    <td style="white-space: nowrap; text-align: right">66.57 ns</td>
    <td style="white-space: nowrap; text-align: right">±631.41%</td>
    <td style="white-space: nowrap; text-align: right">53 ns</td>
    <td style="white-space: nowrap; text-align: right">102.86 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">2.94 M</td>
    <td style="white-space: nowrap; text-align: right">339.70 ns</td>
    <td style="white-space: nowrap; text-align: right">±222.09%</td>
    <td style="white-space: nowrap; text-align: right">251 ns</td>
    <td style="white-space: nowrap; text-align: right">1811.20 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">2.26 M</td>
    <td style="white-space: nowrap; text-align: right">442.64 ns</td>
    <td style="white-space: nowrap; text-align: right">±385.09%</td>
    <td style="white-space: nowrap; text-align: right">257.50 ns</td>
    <td style="white-space: nowrap; text-align: right">10374.74 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">list ++ [val] (list)</td>
    <td style="white-space: nowrap; text-align: right">0.46 M</td>
    <td style="white-space: nowrap; text-align: right">2153.84 ns</td>
    <td style="white-space: nowrap; text-align: right">±114.50%</td>
    <td style="white-space: nowrap; text-align: right">1360 ns</td>
    <td style="white-space: nowrap; text-align: right">9074.82 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.43 M</td>
    <td style="white-space: nowrap; text-align: right">2343.52 ns</td>
    <td style="white-space: nowrap; text-align: right">±138.37%</td>
    <td style="white-space: nowrap; text-align: right">2046 ns</td>
    <td style="white-space: nowrap; text-align: right">15088.36 ns</td>
  </tr>

</table>


Comparison

<table style="width: 1%">
  <tr>
    <th>Name</th>
    <th style="text-align: right">IPS</th>
    <th style="text-align: right">Slower</th>
  <tr>
    <td style="white-space: nowrap">[val | list] (list, backwards)</td>
    <td style="white-space: nowrap;text-align: right">15.02 M</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">2.94 M</td>
    <td style="white-space: nowrap; text-align: right">5.1x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">2.26 M</td>
    <td style="white-space: nowrap; text-align: right">6.65x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">list ++ [val] (list)</td>
    <td style="white-space: nowrap; text-align: right">0.46 M</td>
    <td style="white-space: nowrap; text-align: right">32.35x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.43 M</td>
    <td style="white-space: nowrap; text-align: right">35.2x</td>
  </tr>

</table>



Memory Usage

<table style="width: 1%">
  <tr>
    <th>Name</th>
    <th style="text-align: right">Memory</th>
<th style="text-align: right">Factor</th>
  </tr>
  <tr>
    <td style="white-space: nowrap">[val | list] (list, backwards)</td>
    <td style="white-space: nowrap">16 B</td>
<td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (ErlangArray)</td>
    <td style="white-space: nowrap">368 B</td>
    <td>23.0x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (MapArray)</td>
    <td style="white-space: nowrap">336 B</td>
    <td>21.0x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">list ++ [val] (list)</td>
    <td style="white-space: nowrap">4192 B</td>
    <td>262.0x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (RRBVector)</td>
    <td style="white-space: nowrap">88 B</td>
    <td>5.5x</td>
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
    <td style="white-space: nowrap">[val | list] (list, backwards)</td>
    <td style="white-space: nowrap; text-align: right">11634.04 K</td>
    <td style="white-space: nowrap; text-align: right">0.0860 μs</td>
    <td style="white-space: nowrap; text-align: right">±375.65%</td>
    <td style="white-space: nowrap; text-align: right">0.0650 μs</td>
    <td style="white-space: nowrap; text-align: right">0.33 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">2241.18 K</td>
    <td style="white-space: nowrap; text-align: right">0.45 μs</td>
    <td style="white-space: nowrap; text-align: right">±213.46%</td>
    <td style="white-space: nowrap; text-align: right">0.32 μs</td>
    <td style="white-space: nowrap; text-align: right">2.01 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">951.57 K</td>
    <td style="white-space: nowrap; text-align: right">1.05 μs</td>
    <td style="white-space: nowrap; text-align: right">±360.56%</td>
    <td style="white-space: nowrap; text-align: right">0.29 μs</td>
    <td style="white-space: nowrap; text-align: right">14.01 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">246.34 K</td>
    <td style="white-space: nowrap; text-align: right">4.06 μs</td>
    <td style="white-space: nowrap; text-align: right">±370.70%</td>
    <td style="white-space: nowrap; text-align: right">2.26 μs</td>
    <td style="white-space: nowrap; text-align: right">138.41 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">list ++ [val] (list)</td>
    <td style="white-space: nowrap; text-align: right">124.43 K</td>
    <td style="white-space: nowrap; text-align: right">8.04 μs</td>
    <td style="white-space: nowrap; text-align: right">±106.36%</td>
    <td style="white-space: nowrap; text-align: right">3.14 μs</td>
    <td style="white-space: nowrap; text-align: right">37.37 μs</td>
  </tr>

</table>


Comparison

<table style="width: 1%">
  <tr>
    <th>Name</th>
    <th style="text-align: right">IPS</th>
    <th style="text-align: right">Slower</th>
  <tr>
    <td style="white-space: nowrap">[val | list] (list, backwards)</td>
    <td style="white-space: nowrap;text-align: right">11634.04 K</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">2241.18 K</td>
    <td style="white-space: nowrap; text-align: right">5.19x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">951.57 K</td>
    <td style="white-space: nowrap; text-align: right">12.23x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">246.34 K</td>
    <td style="white-space: nowrap; text-align: right">47.23x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">list ++ [val] (list)</td>
    <td style="white-space: nowrap; text-align: right">124.43 K</td>
    <td style="white-space: nowrap; text-align: right">93.5x</td>
  </tr>

</table>



Memory Usage

<table style="width: 1%">
  <tr>
    <th>Name</th>
    <th style="text-align: right">Memory</th>
<th style="text-align: right">Factor</th>
  </tr>
  <tr>
    <td style="white-space: nowrap">[val | list] (list, backwards)</td>
    <td style="white-space: nowrap">16 B</td>
<td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (ErlangArray)</td>
    <td style="white-space: nowrap">464 B</td>
    <td>29.0x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (MapArray)</td>
    <td style="white-space: nowrap">368 B</td>
    <td>23.0x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (RRBVector)</td>
    <td style="white-space: nowrap">88 B</td>
    <td>5.5x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">list ++ [val] (list)</td>
    <td style="white-space: nowrap">3904 B</td>
    <td>244.0x</td>
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
    <td style="white-space: nowrap">[val | list] (list, backwards)</td>
    <td style="white-space: nowrap; text-align: right">9357.05 K</td>
    <td style="white-space: nowrap; text-align: right">0.107 μs</td>
    <td style="white-space: nowrap; text-align: right">±601.72%</td>
    <td style="white-space: nowrap; text-align: right">0.0760 μs</td>
    <td style="white-space: nowrap; text-align: right">0.22 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">2354.63 K</td>
    <td style="white-space: nowrap; text-align: right">0.42 μs</td>
    <td style="white-space: nowrap; text-align: right">±341.56%</td>
    <td style="white-space: nowrap; text-align: right">0.30 μs</td>
    <td style="white-space: nowrap; text-align: right">0.83 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">961.41 K</td>
    <td style="white-space: nowrap; text-align: right">1.04 μs</td>
    <td style="white-space: nowrap; text-align: right">±210.89%</td>
    <td style="white-space: nowrap; text-align: right">0.35 μs</td>
    <td style="white-space: nowrap; text-align: right">7.31 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">329.05 K</td>
    <td style="white-space: nowrap; text-align: right">3.04 μs</td>
    <td style="white-space: nowrap; text-align: right">±85.31%</td>
    <td style="white-space: nowrap; text-align: right">2.58 μs</td>
    <td style="white-space: nowrap; text-align: right">21.71 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">list ++ [val] (list)</td>
    <td style="white-space: nowrap; text-align: right">61.56 K</td>
    <td style="white-space: nowrap; text-align: right">16.25 μs</td>
    <td style="white-space: nowrap; text-align: right">±82.26%</td>
    <td style="white-space: nowrap; text-align: right">6.57 μs</td>
    <td style="white-space: nowrap; text-align: right">65.38 μs</td>
  </tr>

</table>


Comparison

<table style="width: 1%">
  <tr>
    <th>Name</th>
    <th style="text-align: right">IPS</th>
    <th style="text-align: right">Slower</th>
  <tr>
    <td style="white-space: nowrap">[val | list] (list, backwards)</td>
    <td style="white-space: nowrap;text-align: right">9357.05 K</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">2354.63 K</td>
    <td style="white-space: nowrap; text-align: right">3.97x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">961.41 K</td>
    <td style="white-space: nowrap; text-align: right">9.73x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">329.05 K</td>
    <td style="white-space: nowrap; text-align: right">28.44x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">list ++ [val] (list)</td>
    <td style="white-space: nowrap; text-align: right">61.56 K</td>
    <td style="white-space: nowrap; text-align: right">152.01x</td>
  </tr>

</table>



Memory Usage

<table style="width: 1%">
  <tr>
    <th>Name</th>
    <th style="text-align: right">Memory</th>
<th style="text-align: right">Factor</th>
  </tr>
  <tr>
    <td style="white-space: nowrap">[val | list] (list, backwards)</td>
    <td style="white-space: nowrap">16 B</td>
<td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (MapArray)</td>
    <td style="white-space: nowrap">408 B</td>
    <td>25.5x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (ErlangArray)</td>
    <td style="white-space: nowrap">464 B</td>
    <td>29.0x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (RRBVector)</td>
    <td style="white-space: nowrap">88 B</td>
    <td>5.5x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">list ++ [val] (list)</td>
    <td style="white-space: nowrap">320 B</td>
    <td>20.0x</td>
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
    <td style="white-space: nowrap">[val | list] (list, backwards)</td>
    <td style="white-space: nowrap; text-align: right">6665.20 K</td>
    <td style="white-space: nowrap; text-align: right">0.150 μs</td>
    <td style="white-space: nowrap; text-align: right">±110.24%</td>
    <td style="white-space: nowrap; text-align: right">0.128 μs</td>
    <td style="white-space: nowrap; text-align: right">0.42 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">1242.62 K</td>
    <td style="white-space: nowrap; text-align: right">0.80 μs</td>
    <td style="white-space: nowrap; text-align: right">±235.11%</td>
    <td style="white-space: nowrap; text-align: right">0.39 μs</td>
    <td style="white-space: nowrap; text-align: right">15.97 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">244.49 K</td>
    <td style="white-space: nowrap; text-align: right">4.09 μs</td>
    <td style="white-space: nowrap; text-align: right">±40.72%</td>
    <td style="white-space: nowrap; text-align: right">3.80 μs</td>
    <td style="white-space: nowrap; text-align: right">17.66 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">230.45 K</td>
    <td style="white-space: nowrap; text-align: right">4.34 μs</td>
    <td style="white-space: nowrap; text-align: right">±310.95%</td>
    <td style="white-space: nowrap; text-align: right">0.37 μs</td>
    <td style="white-space: nowrap; text-align: right">61.67 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">list ++ [val] (list)</td>
    <td style="white-space: nowrap; text-align: right">28.82 K</td>
    <td style="white-space: nowrap; text-align: right">34.70 μs</td>
    <td style="white-space: nowrap; text-align: right">±64.47%</td>
    <td style="white-space: nowrap; text-align: right">26.31 μs</td>
    <td style="white-space: nowrap; text-align: right">127.07 μs</td>
  </tr>

</table>


Comparison

<table style="width: 1%">
  <tr>
    <th>Name</th>
    <th style="text-align: right">IPS</th>
    <th style="text-align: right">Slower</th>
  <tr>
    <td style="white-space: nowrap">[val | list] (list, backwards)</td>
    <td style="white-space: nowrap;text-align: right">6665.20 K</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">1242.62 K</td>
    <td style="white-space: nowrap; text-align: right">5.36x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">244.49 K</td>
    <td style="white-space: nowrap; text-align: right">27.26x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">230.45 K</td>
    <td style="white-space: nowrap; text-align: right">28.92x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">list ++ [val] (list)</td>
    <td style="white-space: nowrap; text-align: right">28.82 K</td>
    <td style="white-space: nowrap; text-align: right">231.29x</td>
  </tr>

</table>



Memory Usage

<table style="width: 1%">
  <tr>
    <th>Name</th>
    <th style="text-align: right">Memory</th>
<th style="text-align: right">Factor</th>
  </tr>
  <tr>
    <td style="white-space: nowrap">[val | list] (list, backwards)</td>
    <td style="white-space: nowrap">16 B</td>
<td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (ErlangArray)</td>
    <td style="white-space: nowrap">464 B</td>
    <td>29.0x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (RRBVector)</td>
    <td style="white-space: nowrap">88 B</td>
    <td>5.5x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (MapArray)</td>
    <td style="white-space: nowrap">400 B</td>
    <td>25.0x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">list ++ [val] (list)</td>
    <td style="white-space: nowrap">21728 B</td>
    <td>1358.0x</td>
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
    <td style="white-space: nowrap">[val | list] (list, backwards)</td>
    <td style="white-space: nowrap; text-align: right">4.01 M</td>
    <td style="white-space: nowrap; text-align: right">249.24 ns</td>
    <td style="white-space: nowrap; text-align: right">±53.74%</td>
    <td style="white-space: nowrap; text-align: right">233 ns</td>
    <td style="white-space: nowrap; text-align: right">1068.70 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">1.64 M</td>
    <td style="white-space: nowrap; text-align: right">608.45 ns</td>
    <td style="white-space: nowrap; text-align: right">±29.04%</td>
    <td style="white-space: nowrap; text-align: right">543 ns</td>
    <td style="white-space: nowrap; text-align: right">1314 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">1.18 M</td>
    <td style="white-space: nowrap; text-align: right">846.05 ns</td>
    <td style="white-space: nowrap; text-align: right">±254.12%</td>
    <td style="white-space: nowrap; text-align: right">571 ns</td>
    <td style="white-space: nowrap; text-align: right">20439 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.159 M</td>
    <td style="white-space: nowrap; text-align: right">6295.38 ns</td>
    <td style="white-space: nowrap; text-align: right">±70.65%</td>
    <td style="white-space: nowrap; text-align: right">4940 ns</td>
    <td style="white-space: nowrap; text-align: right">24211 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">list ++ [val] (list)</td>
    <td style="white-space: nowrap; text-align: right">0.0151 M</td>
    <td style="white-space: nowrap; text-align: right">66170.86 ns</td>
    <td style="white-space: nowrap; text-align: right">±74.33%</td>
    <td style="white-space: nowrap; text-align: right">26708 ns</td>
    <td style="white-space: nowrap; text-align: right">228273.32 ns</td>
  </tr>

</table>


Comparison

<table style="width: 1%">
  <tr>
    <th>Name</th>
    <th style="text-align: right">IPS</th>
    <th style="text-align: right">Slower</th>
  <tr>
    <td style="white-space: nowrap">[val | list] (list, backwards)</td>
    <td style="white-space: nowrap;text-align: right">4.01 M</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">1.64 M</td>
    <td style="white-space: nowrap; text-align: right">2.44x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">1.18 M</td>
    <td style="white-space: nowrap; text-align: right">3.39x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.159 M</td>
    <td style="white-space: nowrap; text-align: right">25.26x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">list ++ [val] (list)</td>
    <td style="white-space: nowrap; text-align: right">0.0151 M</td>
    <td style="white-space: nowrap; text-align: right">265.49x</td>
  </tr>

</table>



Memory Usage

<table style="width: 1%">
  <tr>
    <th>Name</th>
    <th style="text-align: right">Memory</th>
<th style="text-align: right">Factor</th>
  </tr>
  <tr>
    <td style="white-space: nowrap">[val | list] (list, backwards)</td>
    <td style="white-space: nowrap">16 B</td>
<td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (MapArray)</td>
    <td style="white-space: nowrap">488 B</td>
    <td>30.5x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (ErlangArray)</td>
    <td style="white-space: nowrap">464 B</td>
    <td>29.0x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (RRBVector)</td>
    <td style="white-space: nowrap">88 B</td>
    <td>5.5x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">list ++ [val] (list)</td>
    <td style="white-space: nowrap">10384 B</td>
    <td>649.0x</td>
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
    <td style="white-space: nowrap">[val | list] (list, backwards)</td>
    <td style="white-space: nowrap; text-align: right">1391.89 K</td>
    <td style="white-space: nowrap; text-align: right">0.72 μs</td>
    <td style="white-space: nowrap; text-align: right">±103.50%</td>
    <td style="white-space: nowrap; text-align: right">0.60 μs</td>
    <td style="white-space: nowrap; text-align: right">6.49 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">971.57 K</td>
    <td style="white-space: nowrap; text-align: right">1.03 μs</td>
    <td style="white-space: nowrap; text-align: right">±16.25%</td>
    <td style="white-space: nowrap; text-align: right">1.01 μs</td>
    <td style="white-space: nowrap; text-align: right">1.61 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">917.33 K</td>
    <td style="white-space: nowrap; text-align: right">1.09 μs</td>
    <td style="white-space: nowrap; text-align: right">±20.75%</td>
    <td style="white-space: nowrap; text-align: right">1.05 μs</td>
    <td style="white-space: nowrap; text-align: right">2.00 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">192.97 K</td>
    <td style="white-space: nowrap; text-align: right">5.18 μs</td>
    <td style="white-space: nowrap; text-align: right">±9.18%</td>
    <td style="white-space: nowrap; text-align: right">5.08 μs</td>
    <td style="white-space: nowrap; text-align: right">6.14 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">list ++ [val] (list)</td>
    <td style="white-space: nowrap; text-align: right">5.29 K</td>
    <td style="white-space: nowrap; text-align: right">188.96 μs</td>
    <td style="white-space: nowrap; text-align: right">±76.93%</td>
    <td style="white-space: nowrap; text-align: right">112.73 μs</td>
    <td style="white-space: nowrap; text-align: right">540.04 μs</td>
  </tr>

</table>


Comparison

<table style="width: 1%">
  <tr>
    <th>Name</th>
    <th style="text-align: right">IPS</th>
    <th style="text-align: right">Slower</th>
  <tr>
    <td style="white-space: nowrap">[val | list] (list, backwards)</td>
    <td style="white-space: nowrap;text-align: right">1391.89 K</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">971.57 K</td>
    <td style="white-space: nowrap; text-align: right">1.43x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">917.33 K</td>
    <td style="white-space: nowrap; text-align: right">1.52x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">192.97 K</td>
    <td style="white-space: nowrap; text-align: right">7.21x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">list ++ [val] (list)</td>
    <td style="white-space: nowrap; text-align: right">5.29 K</td>
    <td style="white-space: nowrap; text-align: right">263.01x</td>
  </tr>

</table>



Memory Usage

<table style="width: 1%">
  <tr>
    <th>Name</th>
    <th style="text-align: right">Memory</th>
<th style="text-align: right">Factor</th>
  </tr>
  <tr>
    <td style="white-space: nowrap">[val | list] (list, backwards)</td>
    <td style="white-space: nowrap">16 B</td>
<td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (ErlangArray)</td>
    <td style="white-space: nowrap">560 B</td>
    <td>35.0x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (MapArray)</td>
    <td style="white-space: nowrap">520 B</td>
    <td>32.5x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (RRBVector)</td>
    <td style="white-space: nowrap">88 B</td>
    <td>5.5x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">list ++ [val] (list)</td>
    <td style="white-space: nowrap">108832 B</td>
    <td>6802.0x</td>
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
    <td style="white-space: nowrap">[val | list] (list, backwards)</td>
    <td style="white-space: nowrap; text-align: right">1159.88 K</td>
    <td style="white-space: nowrap; text-align: right">0.86 μs</td>
    <td style="white-space: nowrap; text-align: right">±13.34%</td>
    <td style="white-space: nowrap; text-align: right">0.84 μs</td>
    <td style="white-space: nowrap; text-align: right">1.34 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">910.97 K</td>
    <td style="white-space: nowrap; text-align: right">1.10 μs</td>
    <td style="white-space: nowrap; text-align: right">±8.11%</td>
    <td style="white-space: nowrap; text-align: right">1.08 μs</td>
    <td style="white-space: nowrap; text-align: right">1.39 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">892.20 K</td>
    <td style="white-space: nowrap; text-align: right">1.12 μs</td>
    <td style="white-space: nowrap; text-align: right">±10.53%</td>
    <td style="white-space: nowrap; text-align: right">1.08 μs</td>
    <td style="white-space: nowrap; text-align: right">1.39 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">175.00 K</td>
    <td style="white-space: nowrap; text-align: right">5.71 μs</td>
    <td style="white-space: nowrap; text-align: right">±12.53%</td>
    <td style="white-space: nowrap; text-align: right">5.36 μs</td>
    <td style="white-space: nowrap; text-align: right">7.30 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">list ++ [val] (list)</td>
    <td style="white-space: nowrap; text-align: right">3.79 K</td>
    <td style="white-space: nowrap; text-align: right">263.89 μs</td>
    <td style="white-space: nowrap; text-align: right">±81.27%</td>
    <td style="white-space: nowrap; text-align: right">113.85 μs</td>
    <td style="white-space: nowrap; text-align: right">598.50 μs</td>
  </tr>

</table>


Comparison

<table style="width: 1%">
  <tr>
    <th>Name</th>
    <th style="text-align: right">IPS</th>
    <th style="text-align: right">Slower</th>
  <tr>
    <td style="white-space: nowrap">[val | list] (list, backwards)</td>
    <td style="white-space: nowrap;text-align: right">1159.88 K</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">910.97 K</td>
    <td style="white-space: nowrap; text-align: right">1.27x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">892.20 K</td>
    <td style="white-space: nowrap; text-align: right">1.3x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">175.00 K</td>
    <td style="white-space: nowrap; text-align: right">6.63x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">list ++ [val] (list)</td>
    <td style="white-space: nowrap; text-align: right">3.79 K</td>
    <td style="white-space: nowrap; text-align: right">306.08x</td>
  </tr>

</table>



Memory Usage

<table style="width: 1%">
  <tr>
    <th>Name</th>
    <th style="text-align: right">Memory</th>
<th style="text-align: right">Factor</th>
  </tr>
  <tr>
    <td style="white-space: nowrap">[val | list] (list, backwards)</td>
    <td style="white-space: nowrap">16 B</td>
<td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (ErlangArray)</td>
    <td style="white-space: nowrap">560 B</td>
    <td>35.0x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (MapArray)</td>
    <td style="white-space: nowrap">552 B</td>
    <td>34.5x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (RRBVector)</td>
    <td style="white-space: nowrap">88 B</td>
    <td>5.5x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">list ++ [val] (list)</td>
    <td style="white-space: nowrap">76224 B</td>
    <td>4764.0x</td>
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
    <td style="white-space: nowrap">[val | list] (list, backwards)</td>
    <td style="white-space: nowrap; text-align: right">1107.52 K</td>
    <td style="white-space: nowrap; text-align: right">0.90 μs</td>
    <td style="white-space: nowrap; text-align: right">±20.85%</td>
    <td style="white-space: nowrap; text-align: right">0.85 μs</td>
    <td style="white-space: nowrap; text-align: right">1.43 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">750.89 K</td>
    <td style="white-space: nowrap; text-align: right">1.33 μs</td>
    <td style="white-space: nowrap; text-align: right">±10.59%</td>
    <td style="white-space: nowrap; text-align: right">1.32 μs</td>
    <td style="white-space: nowrap; text-align: right">1.65 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">699.72 K</td>
    <td style="white-space: nowrap; text-align: right">1.43 μs</td>
    <td style="white-space: nowrap; text-align: right">±11.85%</td>
    <td style="white-space: nowrap; text-align: right">1.39 μs</td>
    <td style="white-space: nowrap; text-align: right">1.75 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">175.79 K</td>
    <td style="white-space: nowrap; text-align: right">5.69 μs</td>
    <td style="white-space: nowrap; text-align: right">±2.50%</td>
    <td style="white-space: nowrap; text-align: right">5.68 μs</td>
    <td style="white-space: nowrap; text-align: right">5.85 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">list ++ [val] (list)</td>
    <td style="white-space: nowrap; text-align: right">3.16 K</td>
    <td style="white-space: nowrap; text-align: right">316.46 μs</td>
    <td style="white-space: nowrap; text-align: right">±63.84%</td>
    <td style="white-space: nowrap; text-align: right">243.57 μs</td>
    <td style="white-space: nowrap; text-align: right">894.52 μs</td>
  </tr>

</table>


Comparison

<table style="width: 1%">
  <tr>
    <th>Name</th>
    <th style="text-align: right">IPS</th>
    <th style="text-align: right">Slower</th>
  <tr>
    <td style="white-space: nowrap">[val | list] (list, backwards)</td>
    <td style="white-space: nowrap;text-align: right">1107.52 K</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">750.89 K</td>
    <td style="white-space: nowrap; text-align: right">1.47x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">699.72 K</td>
    <td style="white-space: nowrap; text-align: right">1.58x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">175.79 K</td>
    <td style="white-space: nowrap; text-align: right">6.3x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">list ++ [val] (list)</td>
    <td style="white-space: nowrap; text-align: right">3.16 K</td>
    <td style="white-space: nowrap; text-align: right">350.48x</td>
  </tr>

</table>



Memory Usage

<table style="width: 1%">
  <tr>
    <th>Name</th>
    <th style="text-align: right">Memory</th>
<th style="text-align: right">Factor</th>
  </tr>
  <tr>
    <td style="white-space: nowrap">[val | list] (list, backwards)</td>
    <td style="white-space: nowrap">16 B</td>
<td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (ErlangArray)</td>
    <td style="white-space: nowrap">560 B</td>
    <td>35.0x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (MapArray)</td>
    <td style="white-space: nowrap">592 B</td>
    <td>37.0x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (RRBVector)</td>
    <td style="white-space: nowrap">88 B</td>
    <td>5.5x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">list ++ [val] (list)</td>
    <td style="white-space: nowrap">524224 B</td>
    <td>32764.0x</td>
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
    <td style="white-space: nowrap">[val | list] (list, backwards)</td>
    <td style="white-space: nowrap; text-align: right">1015.64 K</td>
    <td style="white-space: nowrap; text-align: right">0.98 μs</td>
    <td style="white-space: nowrap; text-align: right">±17.83%</td>
    <td style="white-space: nowrap; text-align: right">0.88 μs</td>
    <td style="white-space: nowrap; text-align: right">1.18 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">815.49 K</td>
    <td style="white-space: nowrap; text-align: right">1.23 μs</td>
    <td style="white-space: nowrap; text-align: right">±3.89%</td>
    <td style="white-space: nowrap; text-align: right">1.24 μs</td>
    <td style="white-space: nowrap; text-align: right">1.26 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">759.11 K</td>
    <td style="white-space: nowrap; text-align: right">1.32 μs</td>
    <td style="white-space: nowrap; text-align: right">±7.62%</td>
    <td style="white-space: nowrap; text-align: right">1.30 μs</td>
    <td style="white-space: nowrap; text-align: right">1.43 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">163.41 K</td>
    <td style="white-space: nowrap; text-align: right">6.12 μs</td>
    <td style="white-space: nowrap; text-align: right">±0.40%</td>
    <td style="white-space: nowrap; text-align: right">6.12 μs</td>
    <td style="white-space: nowrap; text-align: right">6.14 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">list ++ [val] (list)</td>
    <td style="white-space: nowrap; text-align: right">1.04 K</td>
    <td style="white-space: nowrap; text-align: right">964.27 μs</td>
    <td style="white-space: nowrap; text-align: right">±84.82%</td>
    <td style="white-space: nowrap; text-align: right">584.38 μs</td>
    <td style="white-space: nowrap; text-align: right">2424.51 μs</td>
  </tr>

</table>


Comparison

<table style="width: 1%">
  <tr>
    <th>Name</th>
    <th style="text-align: right">IPS</th>
    <th style="text-align: right">Slower</th>
  <tr>
    <td style="white-space: nowrap">[val | list] (list, backwards)</td>
    <td style="white-space: nowrap;text-align: right">1015.64 K</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">815.49 K</td>
    <td style="white-space: nowrap; text-align: right">1.25x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">759.11 K</td>
    <td style="white-space: nowrap; text-align: right">1.34x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">163.41 K</td>
    <td style="white-space: nowrap; text-align: right">6.22x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">list ++ [val] (list)</td>
    <td style="white-space: nowrap; text-align: right">1.04 K</td>
    <td style="white-space: nowrap; text-align: right">979.35x</td>
  </tr>

</table>



Memory Usage

<table style="width: 1%">
  <tr>
    <th>Name</th>
    <th style="text-align: right">Memory</th>
<th style="text-align: right">Factor</th>
  </tr>
  <tr>
    <td style="white-space: nowrap">[val | list] (list, backwards)</td>
    <td style="white-space: nowrap">16 B</td>
<td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (ErlangArray)</td>
    <td style="white-space: nowrap">656 B</td>
    <td>41.0x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (MapArray)</td>
    <td style="white-space: nowrap">648 B</td>
    <td>40.5x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (RRBVector)</td>
    <td style="white-space: nowrap">88 B</td>
    <td>5.5x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">list ++ [val] (list)</td>
    <td style="white-space: nowrap">447952 B</td>
    <td>27997.0x</td>
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
    <td style="white-space: nowrap">Arrays.append/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">860.22 K</td>
    <td style="white-space: nowrap; text-align: right">1.16 μs</td>
    <td style="white-space: nowrap; text-align: right">±4.32%</td>
    <td style="white-space: nowrap; text-align: right">1.16 μs</td>
    <td style="white-space: nowrap; text-align: right">1.20 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">[val | list] (list, backwards)</td>
    <td style="white-space: nowrap; text-align: right">850.70 K</td>
    <td style="white-space: nowrap; text-align: right">1.18 μs</td>
    <td style="white-space: nowrap; text-align: right">±3.31%</td>
    <td style="white-space: nowrap; text-align: right">1.18 μs</td>
    <td style="white-space: nowrap; text-align: right">1.20 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">696.14 K</td>
    <td style="white-space: nowrap; text-align: right">1.44 μs</td>
    <td style="white-space: nowrap; text-align: right">±2.41%</td>
    <td style="white-space: nowrap; text-align: right">1.44 μs</td>
    <td style="white-space: nowrap; text-align: right">1.46 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">144.03 K</td>
    <td style="white-space: nowrap; text-align: right">6.94 μs</td>
    <td style="white-space: nowrap; text-align: right">±0.00%</td>
    <td style="white-space: nowrap; text-align: right">6.94 μs</td>
    <td style="white-space: nowrap; text-align: right">6.94 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">list ++ [val] (list)</td>
    <td style="white-space: nowrap; text-align: right">0.25 K</td>
    <td style="white-space: nowrap; text-align: right">4017.70 μs</td>
    <td style="white-space: nowrap; text-align: right">±35.15%</td>
    <td style="white-space: nowrap; text-align: right">4017.70 μs</td>
    <td style="white-space: nowrap; text-align: right">5016.36 μs</td>
  </tr>

</table>


Comparison

<table style="width: 1%">
  <tr>
    <th>Name</th>
    <th style="text-align: right">IPS</th>
    <th style="text-align: right">Slower</th>
  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (ErlangArray)</td>
    <td style="white-space: nowrap;text-align: right">860.22 K</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">[val | list] (list, backwards)</td>
    <td style="white-space: nowrap; text-align: right">850.70 K</td>
    <td style="white-space: nowrap; text-align: right">1.01x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">696.14 K</td>
    <td style="white-space: nowrap; text-align: right">1.24x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">144.03 K</td>
    <td style="white-space: nowrap; text-align: right">5.97x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">list ++ [val] (list)</td>
    <td style="white-space: nowrap; text-align: right">0.25 K</td>
    <td style="white-space: nowrap; text-align: right">3456.08x</td>
  </tr>

</table>



Memory Usage

<table style="width: 1%">
  <tr>
    <th>Name</th>
    <th style="text-align: right">Memory</th>
<th style="text-align: right">Factor</th>
  </tr>
  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (ErlangArray)</td>
    <td style="white-space: nowrap">656 B</td>
<td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">[val | list] (list, backwards)</td>
    <td style="white-space: nowrap">16 B</td>
    <td>0.02x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (MapArray)</td>
    <td style="white-space: nowrap">624 B</td>
    <td>0.95x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (RRBVector)</td>
    <td style="white-space: nowrap">88 B</td>
    <td>0.13x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">list ++ [val] (list)</td>
    <td style="white-space: nowrap">2469504 B</td>
    <td>3764.49x</td>
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
    <td style="white-space: nowrap">Arrays.append/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">851.06 K</td>
    <td style="white-space: nowrap; text-align: right">1.18 μs</td>
    <td style="white-space: nowrap; text-align: right">±0.00%</td>
    <td style="white-space: nowrap; text-align: right">1.18 μs</td>
    <td style="white-space: nowrap; text-align: right">1.18 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">[val | list] (list, backwards)</td>
    <td style="white-space: nowrap; text-align: right">791.77 K</td>
    <td style="white-space: nowrap; text-align: right">1.26 μs</td>
    <td style="white-space: nowrap; text-align: right">±0.00%</td>
    <td style="white-space: nowrap; text-align: right">1.26 μs</td>
    <td style="white-space: nowrap; text-align: right">1.26 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">663.13 K</td>
    <td style="white-space: nowrap; text-align: right">1.51 μs</td>
    <td style="white-space: nowrap; text-align: right">±0.00%</td>
    <td style="white-space: nowrap; text-align: right">1.51 μs</td>
    <td style="white-space: nowrap; text-align: right">1.51 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">43.62 K</td>
    <td style="white-space: nowrap; text-align: right">22.93 μs</td>
    <td style="white-space: nowrap; text-align: right">±0.00%</td>
    <td style="white-space: nowrap; text-align: right">22.93 μs</td>
    <td style="white-space: nowrap; text-align: right">22.93 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">list ++ [val] (list)</td>
    <td style="white-space: nowrap; text-align: right">0.131 K</td>
    <td style="white-space: nowrap; text-align: right">7638.63 μs</td>
    <td style="white-space: nowrap; text-align: right">±0.00%</td>
    <td style="white-space: nowrap; text-align: right">7638.63 μs</td>
    <td style="white-space: nowrap; text-align: right">7638.63 μs</td>
  </tr>

</table>


Comparison

<table style="width: 1%">
  <tr>
    <th>Name</th>
    <th style="text-align: right">IPS</th>
    <th style="text-align: right">Slower</th>
  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (ErlangArray)</td>
    <td style="white-space: nowrap;text-align: right">851.06 K</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">[val | list] (list, backwards)</td>
    <td style="white-space: nowrap; text-align: right">791.77 K</td>
    <td style="white-space: nowrap; text-align: right">1.07x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">663.13 K</td>
    <td style="white-space: nowrap; text-align: right">1.28x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">43.62 K</td>
    <td style="white-space: nowrap; text-align: right">19.51x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">list ++ [val] (list)</td>
    <td style="white-space: nowrap; text-align: right">0.131 K</td>
    <td style="white-space: nowrap; text-align: right">6500.96x</td>
  </tr>

</table>



Memory Usage

<table style="width: 1%">
  <tr>
    <th>Name</th>
    <th style="text-align: right">Memory</th>
<th style="text-align: right">Factor</th>
  </tr>
  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (ErlangArray)</td>
    <td style="white-space: nowrap">656 B</td>
<td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">[val | list] (list, backwards)</td>
    <td style="white-space: nowrap">16 B</td>
    <td>0.02x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (MapArray)</td>
    <td style="white-space: nowrap">648 B</td>
    <td>0.99x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (RRBVector)</td>
    <td style="white-space: nowrap">88 B</td>
    <td>0.13x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">list ++ [val] (list)</td>
    <td style="white-space: nowrap">1207456 B</td>
    <td>1840.63x</td>
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
    <td style="white-space: nowrap">[val | list] (list, backwards)</td>
    <td style="white-space: nowrap; text-align: right">1008.06 K</td>
    <td style="white-space: nowrap; text-align: right">0.99 μs</td>
    <td style="white-space: nowrap; text-align: right">±0.00%</td>
    <td style="white-space: nowrap; text-align: right">0.99 μs</td>
    <td style="white-space: nowrap; text-align: right">0.99 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">765.11 K</td>
    <td style="white-space: nowrap; text-align: right">1.31 μs</td>
    <td style="white-space: nowrap; text-align: right">±0.00%</td>
    <td style="white-space: nowrap; text-align: right">1.31 μs</td>
    <td style="white-space: nowrap; text-align: right">1.31 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">525.76 K</td>
    <td style="white-space: nowrap; text-align: right">1.90 μs</td>
    <td style="white-space: nowrap; text-align: right">±0.00%</td>
    <td style="white-space: nowrap; text-align: right">1.90 μs</td>
    <td style="white-space: nowrap; text-align: right">1.90 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">134.25 K</td>
    <td style="white-space: nowrap; text-align: right">7.45 μs</td>
    <td style="white-space: nowrap; text-align: right">±0.00%</td>
    <td style="white-space: nowrap; text-align: right">7.45 μs</td>
    <td style="white-space: nowrap; text-align: right">7.45 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">list ++ [val] (list)</td>
    <td style="white-space: nowrap; text-align: right">0.0588 K</td>
    <td style="white-space: nowrap; text-align: right">16995.90 μs</td>
    <td style="white-space: nowrap; text-align: right">±0.00%</td>
    <td style="white-space: nowrap; text-align: right">16995.90 μs</td>
    <td style="white-space: nowrap; text-align: right">16995.90 μs</td>
  </tr>

</table>


Comparison

<table style="width: 1%">
  <tr>
    <th>Name</th>
    <th style="text-align: right">IPS</th>
    <th style="text-align: right">Slower</th>
  <tr>
    <td style="white-space: nowrap">[val | list] (list, backwards)</td>
    <td style="white-space: nowrap;text-align: right">1008.06 K</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">765.11 K</td>
    <td style="white-space: nowrap; text-align: right">1.32x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">525.76 K</td>
    <td style="white-space: nowrap; text-align: right">1.92x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">134.25 K</td>
    <td style="white-space: nowrap; text-align: right">7.51x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">list ++ [val] (list)</td>
    <td style="white-space: nowrap; text-align: right">0.0588 K</td>
    <td style="white-space: nowrap; text-align: right">17132.96x</td>
  </tr>

</table>



Memory Usage

<table style="width: 1%">
  <tr>
    <th>Name</th>
    <th style="text-align: right">Memory</th>
<th style="text-align: right">Factor</th>
  </tr>
  <tr>
    <td style="white-space: nowrap">[val | list] (list, backwards)</td>
    <td style="white-space: nowrap">16 B</td>
<td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (ErlangArray)</td>
    <td style="white-space: nowrap">752 B</td>
    <td>47.0x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (MapArray)</td>
    <td style="white-space: nowrap">704 B</td>
    <td>44.0x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (RRBVector)</td>
    <td style="white-space: nowrap">88 B</td>
    <td>5.5x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">list ++ [val] (list)</td>
    <td style="white-space: nowrap">3121584 B</td>
    <td>195099.0x</td>
  </tr>

</table>


<hr/>

