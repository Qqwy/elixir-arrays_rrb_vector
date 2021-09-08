
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
    <td style="white-space: nowrap; text-align: right">21.51 M</td>
    <td style="white-space: nowrap; text-align: right">46.50 ns</td>
    <td style="white-space: nowrap; text-align: right">±1496.86%</td>
    <td style="white-space: nowrap; text-align: right">30 ns</td>
    <td style="white-space: nowrap; text-align: right">70 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">3.24 M</td>
    <td style="white-space: nowrap; text-align: right">309.04 ns</td>
    <td style="white-space: nowrap; text-align: right">±588.16%</td>
    <td style="white-space: nowrap; text-align: right">148 ns</td>
    <td style="white-space: nowrap; text-align: right">728.66 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">list ++ [val] (list)</td>
    <td style="white-space: nowrap; text-align: right">2.31 M</td>
    <td style="white-space: nowrap; text-align: right">432.69 ns</td>
    <td style="white-space: nowrap; text-align: right">±803.05%</td>
    <td style="white-space: nowrap; text-align: right">129 ns</td>
    <td style="white-space: nowrap; text-align: right">4516.18 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">1.07 M</td>
    <td style="white-space: nowrap; text-align: right">931.63 ns</td>
    <td style="white-space: nowrap; text-align: right">±205.92%</td>
    <td style="white-space: nowrap; text-align: right">780 ns</td>
    <td style="white-space: nowrap; text-align: right">1361.77 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">0.35 M</td>
    <td style="white-space: nowrap; text-align: right">2862.57 ns</td>
    <td style="white-space: nowrap; text-align: right">±140.76%</td>
    <td style="white-space: nowrap; text-align: right">2186 ns</td>
    <td style="white-space: nowrap; text-align: right">21657.17 ns</td>
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
    <td style="white-space: nowrap;text-align: right">21.51 M</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">3.24 M</td>
    <td style="white-space: nowrap; text-align: right">6.65x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">list ++ [val] (list)</td>
    <td style="white-space: nowrap; text-align: right">2.31 M</td>
    <td style="white-space: nowrap; text-align: right">9.31x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">1.07 M</td>
    <td style="white-space: nowrap; text-align: right">20.04x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">0.35 M</td>
    <td style="white-space: nowrap; text-align: right">61.56x</td>
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
    <td style="white-space: nowrap; text-align: right">19.50 M</td>
    <td style="white-space: nowrap; text-align: right">51.29 ns</td>
    <td style="white-space: nowrap; text-align: right">±1084.73%</td>
    <td style="white-space: nowrap; text-align: right">33 ns</td>
    <td style="white-space: nowrap; text-align: right">92 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">4.24 M</td>
    <td style="white-space: nowrap; text-align: right">236.11 ns</td>
    <td style="white-space: nowrap; text-align: right">±656.23%</td>
    <td style="white-space: nowrap; text-align: right">138 ns</td>
    <td style="white-space: nowrap; text-align: right">391.84 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">4.15 M</td>
    <td style="white-space: nowrap; text-align: right">241.21 ns</td>
    <td style="white-space: nowrap; text-align: right">±566.50%</td>
    <td style="white-space: nowrap; text-align: right">148 ns</td>
    <td style="white-space: nowrap; text-align: right">600 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">list ++ [val] (list)</td>
    <td style="white-space: nowrap; text-align: right">1.68 M</td>
    <td style="white-space: nowrap; text-align: right">596.68 ns</td>
    <td style="white-space: nowrap; text-align: right">±531.72%</td>
    <td style="white-space: nowrap; text-align: right">219 ns</td>
    <td style="white-space: nowrap; text-align: right">18848.71 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.77 M</td>
    <td style="white-space: nowrap; text-align: right">1305.14 ns</td>
    <td style="white-space: nowrap; text-align: right">±184.77%</td>
    <td style="white-space: nowrap; text-align: right">1056 ns</td>
    <td style="white-space: nowrap; text-align: right">16686.04 ns</td>
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
    <td style="white-space: nowrap;text-align: right">19.50 M</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">4.24 M</td>
    <td style="white-space: nowrap; text-align: right">4.6x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">4.15 M</td>
    <td style="white-space: nowrap; text-align: right">4.7x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">list ++ [val] (list)</td>
    <td style="white-space: nowrap; text-align: right">1.68 M</td>
    <td style="white-space: nowrap; text-align: right">11.63x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.77 M</td>
    <td style="white-space: nowrap; text-align: right">25.45x</td>
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
    <td style="white-space: nowrap">224 B</td>
    <td>14.0x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (ErlangArray)</td>
    <td style="white-space: nowrap">272 B</td>
    <td>17.0x</td>
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
    <td style="white-space: nowrap; text-align: right">15.67 M</td>
    <td style="white-space: nowrap; text-align: right">63.80 ns</td>
    <td style="white-space: nowrap; text-align: right">±983.49%</td>
    <td style="white-space: nowrap; text-align: right">40 ns</td>
    <td style="white-space: nowrap; text-align: right">99.95 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">3.26 M</td>
    <td style="white-space: nowrap; text-align: right">306.94 ns</td>
    <td style="white-space: nowrap; text-align: right">±483.46%</td>
    <td style="white-space: nowrap; text-align: right">187 ns</td>
    <td style="white-space: nowrap; text-align: right">2937.16 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">3.16 M</td>
    <td style="white-space: nowrap; text-align: right">316.45 ns</td>
    <td style="white-space: nowrap; text-align: right">±408.26%</td>
    <td style="white-space: nowrap; text-align: right">199 ns</td>
    <td style="white-space: nowrap; text-align: right">1796.90 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">list ++ [val] (list)</td>
    <td style="white-space: nowrap; text-align: right">1.15 M</td>
    <td style="white-space: nowrap; text-align: right">868.13 ns</td>
    <td style="white-space: nowrap; text-align: right">±382.55%</td>
    <td style="white-space: nowrap; text-align: right">374 ns</td>
    <td style="white-space: nowrap; text-align: right">17554.60 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.90 M</td>
    <td style="white-space: nowrap; text-align: right">1115.50 ns</td>
    <td style="white-space: nowrap; text-align: right">±231.39%</td>
    <td style="white-space: nowrap; text-align: right">856 ns</td>
    <td style="white-space: nowrap; text-align: right">11557 ns</td>
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
    <td style="white-space: nowrap;text-align: right">15.67 M</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">3.26 M</td>
    <td style="white-space: nowrap; text-align: right">4.81x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">3.16 M</td>
    <td style="white-space: nowrap; text-align: right">4.96x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">list ++ [val] (list)</td>
    <td style="white-space: nowrap; text-align: right">1.15 M</td>
    <td style="white-space: nowrap; text-align: right">13.61x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.90 M</td>
    <td style="white-space: nowrap; text-align: right">17.48x</td>
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
    <td style="white-space: nowrap; text-align: right">16.56 M</td>
    <td style="white-space: nowrap; text-align: right">60.40 ns</td>
    <td style="white-space: nowrap; text-align: right">±960.58%</td>
    <td style="white-space: nowrap; text-align: right">45 ns</td>
    <td style="white-space: nowrap; text-align: right">118.80 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">3.85 M</td>
    <td style="white-space: nowrap; text-align: right">259.50 ns</td>
    <td style="white-space: nowrap; text-align: right">±317.63%</td>
    <td style="white-space: nowrap; text-align: right">204 ns</td>
    <td style="white-space: nowrap; text-align: right">636.10 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">3.77 M</td>
    <td style="white-space: nowrap; text-align: right">264.97 ns</td>
    <td style="white-space: nowrap; text-align: right">±245.02%</td>
    <td style="white-space: nowrap; text-align: right">227.50 ns</td>
    <td style="white-space: nowrap; text-align: right">432.86 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.50 M</td>
    <td style="white-space: nowrap; text-align: right">1993.86 ns</td>
    <td style="white-space: nowrap; text-align: right">±134.45%</td>
    <td style="white-space: nowrap; text-align: right">1768 ns</td>
    <td style="white-space: nowrap; text-align: right">3584.28 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">list ++ [val] (list)</td>
    <td style="white-space: nowrap; text-align: right">0.50 M</td>
    <td style="white-space: nowrap; text-align: right">2013.58 ns</td>
    <td style="white-space: nowrap; text-align: right">±258.73%</td>
    <td style="white-space: nowrap; text-align: right">775 ns</td>
    <td style="white-space: nowrap; text-align: right">27631.20 ns</td>
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
    <td style="white-space: nowrap;text-align: right">16.56 M</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">3.85 M</td>
    <td style="white-space: nowrap; text-align: right">4.3x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">3.77 M</td>
    <td style="white-space: nowrap; text-align: right">4.39x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.50 M</td>
    <td style="white-space: nowrap; text-align: right">33.01x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">list ++ [val] (list)</td>
    <td style="white-space: nowrap; text-align: right">0.50 M</td>
    <td style="white-space: nowrap; text-align: right">33.34x</td>
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
    <td style="white-space: nowrap">Arrays.append/2 (RRBVector)</td>
    <td style="white-space: nowrap">88 B</td>
    <td>5.5x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">list ++ [val] (list)</td>
    <td style="white-space: nowrap">384 B</td>
    <td>24.0x</td>
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
    <td style="white-space: nowrap; text-align: right">13.23 M</td>
    <td style="white-space: nowrap; text-align: right">75.59 ns</td>
    <td style="white-space: nowrap; text-align: right">±434.50%</td>
    <td style="white-space: nowrap; text-align: right">62 ns</td>
    <td style="white-space: nowrap; text-align: right">226.91 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">3.14 M</td>
    <td style="white-space: nowrap; text-align: right">318.22 ns</td>
    <td style="white-space: nowrap; text-align: right">±256.49%</td>
    <td style="white-space: nowrap; text-align: right">216 ns</td>
    <td style="white-space: nowrap; text-align: right">1969.72 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">2.44 M</td>
    <td style="white-space: nowrap; text-align: right">409.71 ns</td>
    <td style="white-space: nowrap; text-align: right">±413.14%</td>
    <td style="white-space: nowrap; text-align: right">247 ns</td>
    <td style="white-space: nowrap; text-align: right">9605.72 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.44 M</td>
    <td style="white-space: nowrap; text-align: right">2256.36 ns</td>
    <td style="white-space: nowrap; text-align: right">±116.27%</td>
    <td style="white-space: nowrap; text-align: right">1999 ns</td>
    <td style="white-space: nowrap; text-align: right">15240.28 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">list ++ [val] (list)</td>
    <td style="white-space: nowrap; text-align: right">0.43 M</td>
    <td style="white-space: nowrap; text-align: right">2318.59 ns</td>
    <td style="white-space: nowrap; text-align: right">±111.57%</td>
    <td style="white-space: nowrap; text-align: right">1528 ns</td>
    <td style="white-space: nowrap; text-align: right">9351.40 ns</td>
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
    <td style="white-space: nowrap;text-align: right">13.23 M</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">3.14 M</td>
    <td style="white-space: nowrap; text-align: right">4.21x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">2.44 M</td>
    <td style="white-space: nowrap; text-align: right">5.42x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.44 M</td>
    <td style="white-space: nowrap; text-align: right">29.85x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">list ++ [val] (list)</td>
    <td style="white-space: nowrap; text-align: right">0.43 M</td>
    <td style="white-space: nowrap; text-align: right">30.67x</td>
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
    <td style="white-space: nowrap">Arrays.append/2 (RRBVector)</td>
    <td style="white-space: nowrap">88 B</td>
    <td>5.5x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">list ++ [val] (list)</td>
    <td style="white-space: nowrap">4192 B</td>
    <td>262.0x</td>
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
    <td style="white-space: nowrap; text-align: right">14499.27 K</td>
    <td style="white-space: nowrap; text-align: right">0.0690 μs</td>
    <td style="white-space: nowrap; text-align: right">±47.86%</td>
    <td style="white-space: nowrap; text-align: right">0.0640 μs</td>
    <td style="white-space: nowrap; text-align: right">0.21 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">2255.07 K</td>
    <td style="white-space: nowrap; text-align: right">0.44 μs</td>
    <td style="white-space: nowrap; text-align: right">±170.97%</td>
    <td style="white-space: nowrap; text-align: right">0.30 μs</td>
    <td style="white-space: nowrap; text-align: right">2.18 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">772.86 K</td>
    <td style="white-space: nowrap; text-align: right">1.29 μs</td>
    <td style="white-space: nowrap; text-align: right">±276.31%</td>
    <td style="white-space: nowrap; text-align: right">0.31 μs</td>
    <td style="white-space: nowrap; text-align: right">14.66 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">276.81 K</td>
    <td style="white-space: nowrap; text-align: right">3.61 μs</td>
    <td style="white-space: nowrap; text-align: right">±337.39%</td>
    <td style="white-space: nowrap; text-align: right">2.35 μs</td>
    <td style="white-space: nowrap; text-align: right">25.16 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">list ++ [val] (list)</td>
    <td style="white-space: nowrap; text-align: right">123.56 K</td>
    <td style="white-space: nowrap; text-align: right">8.09 μs</td>
    <td style="white-space: nowrap; text-align: right">±104.98%</td>
    <td style="white-space: nowrap; text-align: right">3.11 μs</td>
    <td style="white-space: nowrap; text-align: right">35.92 μs</td>
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
    <td style="white-space: nowrap;text-align: right">14499.27 K</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">2255.07 K</td>
    <td style="white-space: nowrap; text-align: right">6.43x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">772.86 K</td>
    <td style="white-space: nowrap; text-align: right">18.76x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">276.81 K</td>
    <td style="white-space: nowrap; text-align: right">52.38x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">list ++ [val] (list)</td>
    <td style="white-space: nowrap; text-align: right">123.56 K</td>
    <td style="white-space: nowrap; text-align: right">117.35x</td>
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
    <td style="white-space: nowrap; text-align: right">10.76 M</td>
    <td style="white-space: nowrap; text-align: right">92.93 ns</td>
    <td style="white-space: nowrap; text-align: right">±45.27%</td>
    <td style="white-space: nowrap; text-align: right">86 ns</td>
    <td style="white-space: nowrap; text-align: right">287.24 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">2.18 M</td>
    <td style="white-space: nowrap; text-align: right">458.44 ns</td>
    <td style="white-space: nowrap; text-align: right">±324.49%</td>
    <td style="white-space: nowrap; text-align: right">330 ns</td>
    <td style="white-space: nowrap; text-align: right">768.96 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">1.14 M</td>
    <td style="white-space: nowrap; text-align: right">877.73 ns</td>
    <td style="white-space: nowrap; text-align: right">±135.42%</td>
    <td style="white-space: nowrap; text-align: right">348 ns</td>
    <td style="white-space: nowrap; text-align: right">2690.20 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.32 M</td>
    <td style="white-space: nowrap; text-align: right">3163.10 ns</td>
    <td style="white-space: nowrap; text-align: right">±70.65%</td>
    <td style="white-space: nowrap; text-align: right">2817 ns</td>
    <td style="white-space: nowrap; text-align: right">21468.18 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">list ++ [val] (list)</td>
    <td style="white-space: nowrap; text-align: right">0.0625 M</td>
    <td style="white-space: nowrap; text-align: right">16012.61 ns</td>
    <td style="white-space: nowrap; text-align: right">±81.36%</td>
    <td style="white-space: nowrap; text-align: right">6553 ns</td>
    <td style="white-space: nowrap; text-align: right">59610.20 ns</td>
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
    <td style="white-space: nowrap;text-align: right">10.76 M</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">2.18 M</td>
    <td style="white-space: nowrap; text-align: right">4.93x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">1.14 M</td>
    <td style="white-space: nowrap; text-align: right">9.45x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.32 M</td>
    <td style="white-space: nowrap; text-align: right">34.04x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">list ++ [val] (list)</td>
    <td style="white-space: nowrap; text-align: right">0.0625 M</td>
    <td style="white-space: nowrap; text-align: right">172.31x</td>
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
    <td style="white-space: nowrap; text-align: right">5604.35 K</td>
    <td style="white-space: nowrap; text-align: right">0.178 μs</td>
    <td style="white-space: nowrap; text-align: right">±97.42%</td>
    <td style="white-space: nowrap; text-align: right">0.135 μs</td>
    <td style="white-space: nowrap; text-align: right">0.96 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">2228.62 K</td>
    <td style="white-space: nowrap; text-align: right">0.45 μs</td>
    <td style="white-space: nowrap; text-align: right">±67.33%</td>
    <td style="white-space: nowrap; text-align: right">0.40 μs</td>
    <td style="white-space: nowrap; text-align: right">3.08 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">559.79 K</td>
    <td style="white-space: nowrap; text-align: right">1.79 μs</td>
    <td style="white-space: nowrap; text-align: right">±495.58%</td>
    <td style="white-space: nowrap; text-align: right">0.40 μs</td>
    <td style="white-space: nowrap; text-align: right">62.18 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">230.63 K</td>
    <td style="white-space: nowrap; text-align: right">4.34 μs</td>
    <td style="white-space: nowrap; text-align: right">±70.25%</td>
    <td style="white-space: nowrap; text-align: right">3.88 μs</td>
    <td style="white-space: nowrap; text-align: right">24.86 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">list ++ [val] (list)</td>
    <td style="white-space: nowrap; text-align: right">29.16 K</td>
    <td style="white-space: nowrap; text-align: right">34.29 μs</td>
    <td style="white-space: nowrap; text-align: right">±79.20%</td>
    <td style="white-space: nowrap; text-align: right">26.32 μs</td>
    <td style="white-space: nowrap; text-align: right">137.52 μs</td>
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
    <td style="white-space: nowrap;text-align: right">5604.35 K</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">2228.62 K</td>
    <td style="white-space: nowrap; text-align: right">2.51x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">559.79 K</td>
    <td style="white-space: nowrap; text-align: right">10.01x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">230.63 K</td>
    <td style="white-space: nowrap; text-align: right">24.3x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">list ++ [val] (list)</td>
    <td style="white-space: nowrap; text-align: right">29.16 K</td>
    <td style="white-space: nowrap; text-align: right">192.19x</td>
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
    <td style="white-space: nowrap">400 B</td>
    <td>25.0x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (RRBVector)</td>
    <td style="white-space: nowrap">88 B</td>
    <td>5.5x</td>
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
    <td style="white-space: nowrap; text-align: right">4.47 M</td>
    <td style="white-space: nowrap; text-align: right">223.70 ns</td>
    <td style="white-space: nowrap; text-align: right">±44.04%</td>
    <td style="white-space: nowrap; text-align: right">201 ns</td>
    <td style="white-space: nowrap; text-align: right">604.92 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">1.39 M</td>
    <td style="white-space: nowrap; text-align: right">721.61 ns</td>
    <td style="white-space: nowrap; text-align: right">±26.29%</td>
    <td style="white-space: nowrap; text-align: right">690 ns</td>
    <td style="white-space: nowrap; text-align: right">1412 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">1.34 M</td>
    <td style="white-space: nowrap; text-align: right">745.20 ns</td>
    <td style="white-space: nowrap; text-align: right">±20.87%</td>
    <td style="white-space: nowrap; text-align: right">722 ns</td>
    <td style="white-space: nowrap; text-align: right">1235 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.167 M</td>
    <td style="white-space: nowrap; text-align: right">5985.13 ns</td>
    <td style="white-space: nowrap; text-align: right">±54.45%</td>
    <td style="white-space: nowrap; text-align: right">5088.50 ns</td>
    <td style="white-space: nowrap; text-align: right">21692 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">list ++ [val] (list)</td>
    <td style="white-space: nowrap; text-align: right">0.0120 M</td>
    <td style="white-space: nowrap; text-align: right">83049.26 ns</td>
    <td style="white-space: nowrap; text-align: right">±60.48%</td>
    <td style="white-space: nowrap; text-align: right">109369 ns</td>
    <td style="white-space: nowrap; text-align: right">230770.80 ns</td>
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
    <td style="white-space: nowrap;text-align: right">4.47 M</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">1.39 M</td>
    <td style="white-space: nowrap; text-align: right">3.23x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">1.34 M</td>
    <td style="white-space: nowrap; text-align: right">3.33x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.167 M</td>
    <td style="white-space: nowrap; text-align: right">26.76x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">list ++ [val] (list)</td>
    <td style="white-space: nowrap; text-align: right">0.0120 M</td>
    <td style="white-space: nowrap; text-align: right">371.25x</td>
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
    <td style="white-space: nowrap">488 B</td>
    <td>30.5x</td>
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
    <td style="white-space: nowrap; text-align: right">1905.26 K</td>
    <td style="white-space: nowrap; text-align: right">0.52 μs</td>
    <td style="white-space: nowrap; text-align: right">±28.83%</td>
    <td style="white-space: nowrap; text-align: right">0.51 μs</td>
    <td style="white-space: nowrap; text-align: right">0.93 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">930.00 K</td>
    <td style="white-space: nowrap; text-align: right">1.08 μs</td>
    <td style="white-space: nowrap; text-align: right">±16.25%</td>
    <td style="white-space: nowrap; text-align: right">1.06 μs</td>
    <td style="white-space: nowrap; text-align: right">1.70 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">872.95 K</td>
    <td style="white-space: nowrap; text-align: right">1.15 μs</td>
    <td style="white-space: nowrap; text-align: right">±15.47%</td>
    <td style="white-space: nowrap; text-align: right">1.09 μs</td>
    <td style="white-space: nowrap; text-align: right">1.66 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">137.47 K</td>
    <td style="white-space: nowrap; text-align: right">7.27 μs</td>
    <td style="white-space: nowrap; text-align: right">±70.67%</td>
    <td style="white-space: nowrap; text-align: right">5.30 μs</td>
    <td style="white-space: nowrap; text-align: right">20.86 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">list ++ [val] (list)</td>
    <td style="white-space: nowrap; text-align: right">4.93 K</td>
    <td style="white-space: nowrap; text-align: right">202.64 μs</td>
    <td style="white-space: nowrap; text-align: right">±76.34%</td>
    <td style="white-space: nowrap; text-align: right">117.35 μs</td>
    <td style="white-space: nowrap; text-align: right">615.22 μs</td>
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
    <td style="white-space: nowrap;text-align: right">1905.26 K</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">930.00 K</td>
    <td style="white-space: nowrap; text-align: right">2.05x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">872.95 K</td>
    <td style="white-space: nowrap; text-align: right">2.18x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">137.47 K</td>
    <td style="white-space: nowrap; text-align: right">13.86x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">list ++ [val] (list)</td>
    <td style="white-space: nowrap; text-align: right">4.93 K</td>
    <td style="white-space: nowrap; text-align: right">386.09x</td>
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
    <td style="white-space: nowrap">Arrays.append/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">827.53 K</td>
    <td style="white-space: nowrap; text-align: right">1.21 μs</td>
    <td style="white-space: nowrap; text-align: right">±37.74%</td>
    <td style="white-space: nowrap; text-align: right">1.05 μs</td>
    <td style="white-space: nowrap; text-align: right">2.88 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">[val | list] (list, backwards)</td>
    <td style="white-space: nowrap; text-align: right">736.36 K</td>
    <td style="white-space: nowrap; text-align: right">1.36 μs</td>
    <td style="white-space: nowrap; text-align: right">±241.51%</td>
    <td style="white-space: nowrap; text-align: right">0.76 μs</td>
    <td style="white-space: nowrap; text-align: right">19.32 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">437.52 K</td>
    <td style="white-space: nowrap; text-align: right">2.29 μs</td>
    <td style="white-space: nowrap; text-align: right">±205.36%</td>
    <td style="white-space: nowrap; text-align: right">1.14 μs</td>
    <td style="white-space: nowrap; text-align: right">21.64 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">135.04 K</td>
    <td style="white-space: nowrap; text-align: right">7.41 μs</td>
    <td style="white-space: nowrap; text-align: right">±62.39%</td>
    <td style="white-space: nowrap; text-align: right">5.82 μs</td>
    <td style="white-space: nowrap; text-align: right">18.82 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">list ++ [val] (list)</td>
    <td style="white-space: nowrap; text-align: right">3.34 K</td>
    <td style="white-space: nowrap; text-align: right">299.38 μs</td>
    <td style="white-space: nowrap; text-align: right">±74.32%</td>
    <td style="white-space: nowrap; text-align: right">118.19 μs</td>
    <td style="white-space: nowrap; text-align: right">628.55 μs</td>
  </tr>

</table>


Comparison

<table style="width: 1%">
  <tr>
    <th>Name</th>
    <th style="text-align: right">IPS</th>
    <th style="text-align: right">Slower</th>
  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (MapArray)</td>
    <td style="white-space: nowrap;text-align: right">827.53 K</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">[val | list] (list, backwards)</td>
    <td style="white-space: nowrap; text-align: right">736.36 K</td>
    <td style="white-space: nowrap; text-align: right">1.12x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">437.52 K</td>
    <td style="white-space: nowrap; text-align: right">1.89x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">135.04 K</td>
    <td style="white-space: nowrap; text-align: right">6.13x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">list ++ [val] (list)</td>
    <td style="white-space: nowrap; text-align: right">3.34 K</td>
    <td style="white-space: nowrap; text-align: right">247.75x</td>
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
    <td style="white-space: nowrap">Arrays.append/2 (MapArray)</td>
    <td style="white-space: nowrap">552 B</td>
<td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">[val | list] (list, backwards)</td>
    <td style="white-space: nowrap">16 B</td>
    <td>0.03x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (ErlangArray)</td>
    <td style="white-space: nowrap">560 B</td>
    <td>1.01x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (RRBVector)</td>
    <td style="white-space: nowrap">88 B</td>
    <td>0.16x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">list ++ [val] (list)</td>
    <td style="white-space: nowrap">76224 B</td>
    <td>138.09x</td>
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
    <td style="white-space: nowrap; text-align: right">1092.14 K</td>
    <td style="white-space: nowrap; text-align: right">0.92 μs</td>
    <td style="white-space: nowrap; text-align: right">±18.66%</td>
    <td style="white-space: nowrap; text-align: right">0.86 μs</td>
    <td style="white-space: nowrap; text-align: right">1.14 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">722.22 K</td>
    <td style="white-space: nowrap; text-align: right">1.38 μs</td>
    <td style="white-space: nowrap; text-align: right">±10.48%</td>
    <td style="white-space: nowrap; text-align: right">1.33 μs</td>
    <td style="white-space: nowrap; text-align: right">1.62 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">685.74 K</td>
    <td style="white-space: nowrap; text-align: right">1.46 μs</td>
    <td style="white-space: nowrap; text-align: right">±7.68%</td>
    <td style="white-space: nowrap; text-align: right">1.48 μs</td>
    <td style="white-space: nowrap; text-align: right">1.59 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">124.22 K</td>
    <td style="white-space: nowrap; text-align: right">8.05 μs</td>
    <td style="white-space: nowrap; text-align: right">±47.78%</td>
    <td style="white-space: nowrap; text-align: right">6.45 μs</td>
    <td style="white-space: nowrap; text-align: right">13.74 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">list ++ [val] (list)</td>
    <td style="white-space: nowrap; text-align: right">3.98 K</td>
    <td style="white-space: nowrap; text-align: right">251.09 μs</td>
    <td style="white-space: nowrap; text-align: right">±6.54%</td>
    <td style="white-space: nowrap; text-align: right">250.61 μs</td>
    <td style="white-space: nowrap; text-align: right">286.17 μs</td>
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
    <td style="white-space: nowrap;text-align: right">1092.14 K</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">722.22 K</td>
    <td style="white-space: nowrap; text-align: right">1.51x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">685.74 K</td>
    <td style="white-space: nowrap; text-align: right">1.59x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">124.22 K</td>
    <td style="white-space: nowrap; text-align: right">8.79x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">list ++ [val] (list)</td>
    <td style="white-space: nowrap; text-align: right">3.98 K</td>
    <td style="white-space: nowrap; text-align: right">274.23x</td>
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
    <td style="white-space: nowrap; text-align: right">1029.87 K</td>
    <td style="white-space: nowrap; text-align: right">0.97 μs</td>
    <td style="white-space: nowrap; text-align: right">±20.19%</td>
    <td style="white-space: nowrap; text-align: right">0.95 μs</td>
    <td style="white-space: nowrap; text-align: right">1.22 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">790.67 K</td>
    <td style="white-space: nowrap; text-align: right">1.26 μs</td>
    <td style="white-space: nowrap; text-align: right">±3.96%</td>
    <td style="white-space: nowrap; text-align: right">1.28 μs</td>
    <td style="white-space: nowrap; text-align: right">1.30 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">766.87 K</td>
    <td style="white-space: nowrap; text-align: right">1.30 μs</td>
    <td style="white-space: nowrap; text-align: right">±3.22%</td>
    <td style="white-space: nowrap; text-align: right">1.31 μs</td>
    <td style="white-space: nowrap; text-align: right">1.34 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">167.32 K</td>
    <td style="white-space: nowrap; text-align: right">5.98 μs</td>
    <td style="white-space: nowrap; text-align: right">±10.38%</td>
    <td style="white-space: nowrap; text-align: right">5.98 μs</td>
    <td style="white-space: nowrap; text-align: right">6.42 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">list ++ [val] (list)</td>
    <td style="white-space: nowrap; text-align: right">1.66 K</td>
    <td style="white-space: nowrap; text-align: right">603.41 μs</td>
    <td style="white-space: nowrap; text-align: right">±6.14%</td>
    <td style="white-space: nowrap; text-align: right">585.97 μs</td>
    <td style="white-space: nowrap; text-align: right">668.47 μs</td>
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
    <td style="white-space: nowrap;text-align: right">1029.87 K</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">790.67 K</td>
    <td style="white-space: nowrap; text-align: right">1.3x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">766.87 K</td>
    <td style="white-space: nowrap; text-align: right">1.34x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">167.32 K</td>
    <td style="white-space: nowrap; text-align: right">6.15x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">list ++ [val] (list)</td>
    <td style="white-space: nowrap; text-align: right">1.66 K</td>
    <td style="white-space: nowrap; text-align: right">621.43x</td>
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
    <td style="white-space: nowrap">[val | list] (list, backwards)</td>
    <td style="white-space: nowrap; text-align: right">1036.81 K</td>
    <td style="white-space: nowrap; text-align: right">0.96 μs</td>
    <td style="white-space: nowrap; text-align: right">±9.16%</td>
    <td style="white-space: nowrap; text-align: right">0.96 μs</td>
    <td style="white-space: nowrap; text-align: right">1.03 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">851.06 K</td>
    <td style="white-space: nowrap; text-align: right">1.18 μs</td>
    <td style="white-space: nowrap; text-align: right">±3.49%</td>
    <td style="white-space: nowrap; text-align: right">1.18 μs</td>
    <td style="white-space: nowrap; text-align: right">1.20 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">571.10 K</td>
    <td style="white-space: nowrap; text-align: right">1.75 μs</td>
    <td style="white-space: nowrap; text-align: right">±10.74%</td>
    <td style="white-space: nowrap; text-align: right">1.75 μs</td>
    <td style="white-space: nowrap; text-align: right">1.88 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">159.01 K</td>
    <td style="white-space: nowrap; text-align: right">6.29 μs</td>
    <td style="white-space: nowrap; text-align: right">±0.00%</td>
    <td style="white-space: nowrap; text-align: right">6.29 μs</td>
    <td style="white-space: nowrap; text-align: right">6.29 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">list ++ [val] (list)</td>
    <td style="white-space: nowrap; text-align: right">0.33 K</td>
    <td style="white-space: nowrap; text-align: right">3069.50 μs</td>
    <td style="white-space: nowrap; text-align: right">±4.26%</td>
    <td style="white-space: nowrap; text-align: right">3069.50 μs</td>
    <td style="white-space: nowrap; text-align: right">3162.04 μs</td>
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
    <td style="white-space: nowrap;text-align: right">1036.81 K</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">851.06 K</td>
    <td style="white-space: nowrap; text-align: right">1.22x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">571.10 K</td>
    <td style="white-space: nowrap; text-align: right">1.82x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">159.01 K</td>
    <td style="white-space: nowrap; text-align: right">6.52x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">list ++ [val] (list)</td>
    <td style="white-space: nowrap; text-align: right">0.33 K</td>
    <td style="white-space: nowrap; text-align: right">3182.48x</td>
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
    <td style="white-space: nowrap">624 B</td>
    <td>39.0x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (RRBVector)</td>
    <td style="white-space: nowrap">88 B</td>
    <td>5.5x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">list ++ [val] (list)</td>
    <td style="white-space: nowrap">2469504 B</td>
    <td>154344.0x</td>
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
    <td style="white-space: nowrap">[val | list] (list, backwards)</td>
    <td style="white-space: nowrap; text-align: right">926.78 K</td>
    <td style="white-space: nowrap; text-align: right">1.08 μs</td>
    <td style="white-space: nowrap; text-align: right">±0.00%</td>
    <td style="white-space: nowrap; text-align: right">1.08 μs</td>
    <td style="white-space: nowrap; text-align: right">1.08 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">807.10 K</td>
    <td style="white-space: nowrap; text-align: right">1.24 μs</td>
    <td style="white-space: nowrap; text-align: right">±0.00%</td>
    <td style="white-space: nowrap; text-align: right">1.24 μs</td>
    <td style="white-space: nowrap; text-align: right">1.24 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">591.02 K</td>
    <td style="white-space: nowrap; text-align: right">1.69 μs</td>
    <td style="white-space: nowrap; text-align: right">±0.00%</td>
    <td style="white-space: nowrap; text-align: right">1.69 μs</td>
    <td style="white-space: nowrap; text-align: right">1.69 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">25.34 K</td>
    <td style="white-space: nowrap; text-align: right">39.46 μs</td>
    <td style="white-space: nowrap; text-align: right">±0.00%</td>
    <td style="white-space: nowrap; text-align: right">39.46 μs</td>
    <td style="white-space: nowrap; text-align: right">39.46 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">list ++ [val] (list)</td>
    <td style="white-space: nowrap; text-align: right">0.131 K</td>
    <td style="white-space: nowrap; text-align: right">7638.77 μs</td>
    <td style="white-space: nowrap; text-align: right">±0.00%</td>
    <td style="white-space: nowrap; text-align: right">7638.77 μs</td>
    <td style="white-space: nowrap; text-align: right">7638.77 μs</td>
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
    <td style="white-space: nowrap;text-align: right">926.78 K</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">807.10 K</td>
    <td style="white-space: nowrap; text-align: right">1.15x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">591.02 K</td>
    <td style="white-space: nowrap; text-align: right">1.57x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">25.34 K</td>
    <td style="white-space: nowrap; text-align: right">36.57x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">list ++ [val] (list)</td>
    <td style="white-space: nowrap; text-align: right">0.131 K</td>
    <td style="white-space: nowrap; text-align: right">7079.49x</td>
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
    <td style="white-space: nowrap">1207456 B</td>
    <td>75466.0x</td>
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
    <td style="white-space: nowrap; text-align: right">956.94 K</td>
    <td style="white-space: nowrap; text-align: right">1.04 μs</td>
    <td style="white-space: nowrap; text-align: right">±0.00%</td>
    <td style="white-space: nowrap; text-align: right">1.04 μs</td>
    <td style="white-space: nowrap; text-align: right">1.04 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">580.72 K</td>
    <td style="white-space: nowrap; text-align: right">1.72 μs</td>
    <td style="white-space: nowrap; text-align: right">±0.00%</td>
    <td style="white-space: nowrap; text-align: right">1.72 μs</td>
    <td style="white-space: nowrap; text-align: right">1.72 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">536.77 K</td>
    <td style="white-space: nowrap; text-align: right">1.86 μs</td>
    <td style="white-space: nowrap; text-align: right">±0.00%</td>
    <td style="white-space: nowrap; text-align: right">1.86 μs</td>
    <td style="white-space: nowrap; text-align: right">1.86 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">145.56 K</td>
    <td style="white-space: nowrap; text-align: right">6.87 μs</td>
    <td style="white-space: nowrap; text-align: right">±0.00%</td>
    <td style="white-space: nowrap; text-align: right">6.87 μs</td>
    <td style="white-space: nowrap; text-align: right">6.87 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">list ++ [val] (list)</td>
    <td style="white-space: nowrap; text-align: right">0.0566 K</td>
    <td style="white-space: nowrap; text-align: right">17683.03 μs</td>
    <td style="white-space: nowrap; text-align: right">±0.00%</td>
    <td style="white-space: nowrap; text-align: right">17683.03 μs</td>
    <td style="white-space: nowrap; text-align: right">17683.03 μs</td>
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
    <td style="white-space: nowrap;text-align: right">956.94 K</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">580.72 K</td>
    <td style="white-space: nowrap; text-align: right">1.65x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">536.77 K</td>
    <td style="white-space: nowrap; text-align: right">1.78x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">145.56 K</td>
    <td style="white-space: nowrap; text-align: right">6.57x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">list ++ [val] (list)</td>
    <td style="white-space: nowrap; text-align: right">0.0566 K</td>
    <td style="white-space: nowrap; text-align: right">16921.56x</td>
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

