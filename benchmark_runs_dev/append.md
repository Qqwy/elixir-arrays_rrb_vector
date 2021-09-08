
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
    <td style="white-space: nowrap; text-align: right">13.24 M</td>
    <td style="white-space: nowrap; text-align: right">75.55 ns</td>
    <td style="white-space: nowrap; text-align: right">±1211.69%</td>
    <td style="white-space: nowrap; text-align: right">46 ns</td>
    <td style="white-space: nowrap; text-align: right">120 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">4.09 M</td>
    <td style="white-space: nowrap; text-align: right">244.34 ns</td>
    <td style="white-space: nowrap; text-align: right">±458.57%</td>
    <td style="white-space: nowrap; text-align: right">167 ns</td>
    <td style="white-space: nowrap; text-align: right">602.88 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">list ++ [val] (list)</td>
    <td style="white-space: nowrap; text-align: right">3.24 M</td>
    <td style="white-space: nowrap; text-align: right">308.76 ns</td>
    <td style="white-space: nowrap; text-align: right">±782.01%</td>
    <td style="white-space: nowrap; text-align: right">133 ns</td>
    <td style="white-space: nowrap; text-align: right">717.80 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">0.36 M</td>
    <td style="white-space: nowrap; text-align: right">2775.21 ns</td>
    <td style="white-space: nowrap; text-align: right">±174.42%</td>
    <td style="white-space: nowrap; text-align: right">2075 ns</td>
    <td style="white-space: nowrap; text-align: right">21852.62 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.179 M</td>
    <td style="white-space: nowrap; text-align: right">5593.66 ns</td>
    <td style="white-space: nowrap; text-align: right">±81.87%</td>
    <td style="white-space: nowrap; text-align: right">4728 ns</td>
    <td style="white-space: nowrap; text-align: right">24303.82 ns</td>
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
    <td style="white-space: nowrap;text-align: right">13.24 M</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">4.09 M</td>
    <td style="white-space: nowrap; text-align: right">3.23x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">list ++ [val] (list)</td>
    <td style="white-space: nowrap; text-align: right">3.24 M</td>
    <td style="white-space: nowrap; text-align: right">4.09x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">0.36 M</td>
    <td style="white-space: nowrap; text-align: right">36.74x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.179 M</td>
    <td style="white-space: nowrap; text-align: right">74.04x</td>
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
    <td style="white-space: nowrap">Arrays.append/2 (MapArray)</td>
    <td style="white-space: nowrap">920 B</td>
    <td>57.5x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (RRBVector)</td>
    <td style="white-space: nowrap">88 B</td>
    <td>5.5x</td>
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
    <td style="white-space: nowrap; text-align: right">11.23 M</td>
    <td style="white-space: nowrap; text-align: right">89.08 ns</td>
    <td style="white-space: nowrap; text-align: right">±989.78%</td>
    <td style="white-space: nowrap; text-align: right">57 ns</td>
    <td style="white-space: nowrap; text-align: right">144 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">3.87 M</td>
    <td style="white-space: nowrap; text-align: right">258.53 ns</td>
    <td style="white-space: nowrap; text-align: right">±543.61%</td>
    <td style="white-space: nowrap; text-align: right">169 ns</td>
    <td style="white-space: nowrap; text-align: right">438.31 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">3.73 M</td>
    <td style="white-space: nowrap; text-align: right">267.83 ns</td>
    <td style="white-space: nowrap; text-align: right">±527.54%</td>
    <td style="white-space: nowrap; text-align: right">172 ns</td>
    <td style="white-space: nowrap; text-align: right">625 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">list ++ [val] (list)</td>
    <td style="white-space: nowrap; text-align: right">1.58 M</td>
    <td style="white-space: nowrap; text-align: right">634.02 ns</td>
    <td style="white-space: nowrap; text-align: right">±515.28%</td>
    <td style="white-space: nowrap; text-align: right">227 ns</td>
    <td style="white-space: nowrap; text-align: right">20042.85 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.23 M</td>
    <td style="white-space: nowrap; text-align: right">4335.26 ns</td>
    <td style="white-space: nowrap; text-align: right">±104.30%</td>
    <td style="white-space: nowrap; text-align: right">3785 ns</td>
    <td style="white-space: nowrap; text-align: right">20358.28 ns</td>
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
    <td style="white-space: nowrap;text-align: right">11.23 M</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">3.87 M</td>
    <td style="white-space: nowrap; text-align: right">2.9x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">3.73 M</td>
    <td style="white-space: nowrap; text-align: right">3.01x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">list ++ [val] (list)</td>
    <td style="white-space: nowrap; text-align: right">1.58 M</td>
    <td style="white-space: nowrap; text-align: right">7.12x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.23 M</td>
    <td style="white-space: nowrap; text-align: right">48.67x</td>
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
    <td style="white-space: nowrap; text-align: right">9.81 M</td>
    <td style="white-space: nowrap; text-align: right">101.89 ns</td>
    <td style="white-space: nowrap; text-align: right">±829.98%</td>
    <td style="white-space: nowrap; text-align: right">68 ns</td>
    <td style="white-space: nowrap; text-align: right">122 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">3.03 M</td>
    <td style="white-space: nowrap; text-align: right">329.89 ns</td>
    <td style="white-space: nowrap; text-align: right">±500.36%</td>
    <td style="white-space: nowrap; text-align: right">209 ns</td>
    <td style="white-space: nowrap; text-align: right">3331.50 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">2.92 M</td>
    <td style="white-space: nowrap; text-align: right">342.70 ns</td>
    <td style="white-space: nowrap; text-align: right">±430.09%</td>
    <td style="white-space: nowrap; text-align: right">235 ns</td>
    <td style="white-space: nowrap; text-align: right">1758.52 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">list ++ [val] (list)</td>
    <td style="white-space: nowrap; text-align: right">1.01 M</td>
    <td style="white-space: nowrap; text-align: right">993.89 ns</td>
    <td style="white-space: nowrap; text-align: right">±390.06%</td>
    <td style="white-space: nowrap; text-align: right">413 ns</td>
    <td style="white-space: nowrap; text-align: right">18755.70 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.24 M</td>
    <td style="white-space: nowrap; text-align: right">4212.89 ns</td>
    <td style="white-space: nowrap; text-align: right">±212.89%</td>
    <td style="white-space: nowrap; text-align: right">3265 ns</td>
    <td style="white-space: nowrap; text-align: right">20384.60 ns</td>
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
    <td style="white-space: nowrap;text-align: right">9.81 M</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">3.03 M</td>
    <td style="white-space: nowrap; text-align: right">3.24x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">2.92 M</td>
    <td style="white-space: nowrap; text-align: right">3.36x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">list ++ [val] (list)</td>
    <td style="white-space: nowrap; text-align: right">1.01 M</td>
    <td style="white-space: nowrap; text-align: right">9.75x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.24 M</td>
    <td style="white-space: nowrap; text-align: right">41.35x</td>
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
    <td style="white-space: nowrap; text-align: right">10.22 M</td>
    <td style="white-space: nowrap; text-align: right">97.87 ns</td>
    <td style="white-space: nowrap; text-align: right">±727.34%</td>
    <td style="white-space: nowrap; text-align: right">67 ns</td>
    <td style="white-space: nowrap; text-align: right">235 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">3.71 M</td>
    <td style="white-space: nowrap; text-align: right">269.28 ns</td>
    <td style="white-space: nowrap; text-align: right">±172.52%</td>
    <td style="white-space: nowrap; text-align: right">250 ns</td>
    <td style="white-space: nowrap; text-align: right">447.32 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">3.30 M</td>
    <td style="white-space: nowrap; text-align: right">302.88 ns</td>
    <td style="white-space: nowrap; text-align: right">±267.78%</td>
    <td style="white-space: nowrap; text-align: right">249 ns</td>
    <td style="white-space: nowrap; text-align: right">689.71 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">list ++ [val] (list)</td>
    <td style="white-space: nowrap; text-align: right">0.51 M</td>
    <td style="white-space: nowrap; text-align: right">1943.53 ns</td>
    <td style="white-space: nowrap; text-align: right">±264.40%</td>
    <td style="white-space: nowrap; text-align: right">748 ns</td>
    <td style="white-space: nowrap; text-align: right">26352.68 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.0967 M</td>
    <td style="white-space: nowrap; text-align: right">10336.01 ns</td>
    <td style="white-space: nowrap; text-align: right">±70.49%</td>
    <td style="white-space: nowrap; text-align: right">9228 ns</td>
    <td style="white-space: nowrap; text-align: right">35162.88 ns</td>
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
    <td style="white-space: nowrap;text-align: right">10.22 M</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">3.71 M</td>
    <td style="white-space: nowrap; text-align: right">2.75x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">3.30 M</td>
    <td style="white-space: nowrap; text-align: right">3.09x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">list ++ [val] (list)</td>
    <td style="white-space: nowrap; text-align: right">0.51 M</td>
    <td style="white-space: nowrap; text-align: right">19.86x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.0967 M</td>
    <td style="white-space: nowrap; text-align: right">105.61x</td>
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
    <td style="white-space: nowrap">328 B</td>
    <td>20.5x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (ErlangArray)</td>
    <td style="white-space: nowrap">368 B</td>
    <td>23.0x</td>
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
    <td style="white-space: nowrap; text-align: right">10.30 M</td>
    <td style="white-space: nowrap; text-align: right">97.10 ns</td>
    <td style="white-space: nowrap; text-align: right">±645.64%</td>
    <td style="white-space: nowrap; text-align: right">70 ns</td>
    <td style="white-space: nowrap; text-align: right">155.10 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">2.72 M</td>
    <td style="white-space: nowrap; text-align: right">367.66 ns</td>
    <td style="white-space: nowrap; text-align: right">±215.77%</td>
    <td style="white-space: nowrap; text-align: right">264 ns</td>
    <td style="white-space: nowrap; text-align: right">1936.46 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">2.37 M</td>
    <td style="white-space: nowrap; text-align: right">421.21 ns</td>
    <td style="white-space: nowrap; text-align: right">±343.58%</td>
    <td style="white-space: nowrap; text-align: right">265 ns</td>
    <td style="white-space: nowrap; text-align: right">10256.92 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">list ++ [val] (list)</td>
    <td style="white-space: nowrap; text-align: right">0.46 M</td>
    <td style="white-space: nowrap; text-align: right">2194.39 ns</td>
    <td style="white-space: nowrap; text-align: right">±113.01%</td>
    <td style="white-space: nowrap; text-align: right">1432 ns</td>
    <td style="white-space: nowrap; text-align: right">9578.20 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.0681 M</td>
    <td style="white-space: nowrap; text-align: right">14692.58 ns</td>
    <td style="white-space: nowrap; text-align: right">±29.33%</td>
    <td style="white-space: nowrap; text-align: right">13844 ns</td>
    <td style="white-space: nowrap; text-align: right">41401.80 ns</td>
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
    <td style="white-space: nowrap;text-align: right">10.30 M</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">2.72 M</td>
    <td style="white-space: nowrap; text-align: right">3.79x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">2.37 M</td>
    <td style="white-space: nowrap; text-align: right">4.34x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">list ++ [val] (list)</td>
    <td style="white-space: nowrap; text-align: right">0.46 M</td>
    <td style="white-space: nowrap; text-align: right">22.6x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.0681 M</td>
    <td style="white-space: nowrap; text-align: right">151.31x</td>
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
    <td style="white-space: nowrap; text-align: right">9815.01 K</td>
    <td style="white-space: nowrap; text-align: right">0.102 μs</td>
    <td style="white-space: nowrap; text-align: right">±469.44%</td>
    <td style="white-space: nowrap; text-align: right">0.0850 μs</td>
    <td style="white-space: nowrap; text-align: right">0.184 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">2151.71 K</td>
    <td style="white-space: nowrap; text-align: right">0.46 μs</td>
    <td style="white-space: nowrap; text-align: right">±177.97%</td>
    <td style="white-space: nowrap; text-align: right">0.34 μs</td>
    <td style="white-space: nowrap; text-align: right">2.04 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">754.59 K</td>
    <td style="white-space: nowrap; text-align: right">1.33 μs</td>
    <td style="white-space: nowrap; text-align: right">±286.04%</td>
    <td style="white-space: nowrap; text-align: right">0.31 μs</td>
    <td style="white-space: nowrap; text-align: right">14.10 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">list ++ [val] (list)</td>
    <td style="white-space: nowrap; text-align: right">131.24 K</td>
    <td style="white-space: nowrap; text-align: right">7.62 μs</td>
    <td style="white-space: nowrap; text-align: right">±110.17%</td>
    <td style="white-space: nowrap; text-align: right">2.88 μs</td>
    <td style="white-space: nowrap; text-align: right">35.22 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">67.05 K</td>
    <td style="white-space: nowrap; text-align: right">14.91 μs</td>
    <td style="white-space: nowrap; text-align: right">±17.42%</td>
    <td style="white-space: nowrap; text-align: right">14.33 μs</td>
    <td style="white-space: nowrap; text-align: right">36.60 μs</td>
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
    <td style="white-space: nowrap;text-align: right">9815.01 K</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">2151.71 K</td>
    <td style="white-space: nowrap; text-align: right">4.56x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">754.59 K</td>
    <td style="white-space: nowrap; text-align: right">13.01x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">list ++ [val] (list)</td>
    <td style="white-space: nowrap; text-align: right">131.24 K</td>
    <td style="white-space: nowrap; text-align: right">74.78x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">67.05 K</td>
    <td style="white-space: nowrap; text-align: right">146.37x</td>
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
    <td style="white-space: nowrap">list ++ [val] (list)</td>
    <td style="white-space: nowrap">3904 B</td>
    <td>244.0x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (RRBVector)</td>
    <td style="white-space: nowrap">88 B</td>
    <td>5.5x</td>
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
    <td style="white-space: nowrap; text-align: right">11.65 M</td>
    <td style="white-space: nowrap; text-align: right">85.87 ns</td>
    <td style="white-space: nowrap; text-align: right">±29.42%</td>
    <td style="white-space: nowrap; text-align: right">82 ns</td>
    <td style="white-space: nowrap; text-align: right">157.52 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">1.64 M</td>
    <td style="white-space: nowrap; text-align: right">609.40 ns</td>
    <td style="white-space: nowrap; text-align: right">±366.56%</td>
    <td style="white-space: nowrap; text-align: right">331 ns</td>
    <td style="white-space: nowrap; text-align: right">18569.06 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">1.29 M</td>
    <td style="white-space: nowrap; text-align: right">772.33 ns</td>
    <td style="white-space: nowrap; text-align: right">±98.11%</td>
    <td style="white-space: nowrap; text-align: right">372 ns</td>
    <td style="white-space: nowrap; text-align: right">2501 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">list ++ [val] (list)</td>
    <td style="white-space: nowrap; text-align: right">0.0661 M</td>
    <td style="white-space: nowrap; text-align: right">15125.69 ns</td>
    <td style="white-space: nowrap; text-align: right">±84.15%</td>
    <td style="white-space: nowrap; text-align: right">5856 ns</td>
    <td style="white-space: nowrap; text-align: right">63236.64 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.0592 M</td>
    <td style="white-space: nowrap; text-align: right">16891.53 ns</td>
    <td style="white-space: nowrap; text-align: right">±21.74%</td>
    <td style="white-space: nowrap; text-align: right">16351 ns</td>
    <td style="white-space: nowrap; text-align: right">39010 ns</td>
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
    <td style="white-space: nowrap;text-align: right">11.65 M</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">1.64 M</td>
    <td style="white-space: nowrap; text-align: right">7.1x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">1.29 M</td>
    <td style="white-space: nowrap; text-align: right">8.99x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">list ++ [val] (list)</td>
    <td style="white-space: nowrap; text-align: right">0.0661 M</td>
    <td style="white-space: nowrap; text-align: right">176.15x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.0592 M</td>
    <td style="white-space: nowrap; text-align: right">196.72x</td>
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
    <td style="white-space: nowrap">list ++ [val] (list)</td>
    <td style="white-space: nowrap">320 B</td>
    <td>20.0x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (RRBVector)</td>
    <td style="white-space: nowrap">88 B</td>
    <td>5.5x</td>
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
    <td style="white-space: nowrap; text-align: right">7272.46 K</td>
    <td style="white-space: nowrap; text-align: right">0.138 μs</td>
    <td style="white-space: nowrap; text-align: right">±48.06%</td>
    <td style="white-space: nowrap; text-align: right">0.132 μs</td>
    <td style="white-space: nowrap; text-align: right">0.34 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">1706.64 K</td>
    <td style="white-space: nowrap; text-align: right">0.59 μs</td>
    <td style="white-space: nowrap; text-align: right">±218.75%</td>
    <td style="white-space: nowrap; text-align: right">0.39 μs</td>
    <td style="white-space: nowrap; text-align: right">5.17 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">733.79 K</td>
    <td style="white-space: nowrap; text-align: right">1.36 μs</td>
    <td style="white-space: nowrap; text-align: right">±504.71%</td>
    <td style="white-space: nowrap; text-align: right">0.39 μs</td>
    <td style="white-space: nowrap; text-align: right">52.62 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">38.99 K</td>
    <td style="white-space: nowrap; text-align: right">25.65 μs</td>
    <td style="white-space: nowrap; text-align: right">±52.30%</td>
    <td style="white-space: nowrap; text-align: right">19.68 μs</td>
    <td style="white-space: nowrap; text-align: right">62.86 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">list ++ [val] (list)</td>
    <td style="white-space: nowrap; text-align: right">30.84 K</td>
    <td style="white-space: nowrap; text-align: right">32.42 μs</td>
    <td style="white-space: nowrap; text-align: right">±63.29%</td>
    <td style="white-space: nowrap; text-align: right">25.94 μs</td>
    <td style="white-space: nowrap; text-align: right">103.53 μs</td>
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
    <td style="white-space: nowrap;text-align: right">7272.46 K</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">1706.64 K</td>
    <td style="white-space: nowrap; text-align: right">4.26x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">733.79 K</td>
    <td style="white-space: nowrap; text-align: right">9.91x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">38.99 K</td>
    <td style="white-space: nowrap; text-align: right">186.53x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">list ++ [val] (list)</td>
    <td style="white-space: nowrap; text-align: right">30.84 K</td>
    <td style="white-space: nowrap; text-align: right">235.81x</td>
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
    <td style="white-space: nowrap; text-align: right">3.10 M</td>
    <td style="white-space: nowrap; text-align: right">322.65 ns</td>
    <td style="white-space: nowrap; text-align: right">±467.21%</td>
    <td style="white-space: nowrap; text-align: right">174 ns</td>
    <td style="white-space: nowrap; text-align: right">9973.37 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">1.54 M</td>
    <td style="white-space: nowrap; text-align: right">648.57 ns</td>
    <td style="white-space: nowrap; text-align: right">±21.08%</td>
    <td style="white-space: nowrap; text-align: right">616 ns</td>
    <td style="white-space: nowrap; text-align: right">1217 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">1.50 M</td>
    <td style="white-space: nowrap; text-align: right">666.35 ns</td>
    <td style="white-space: nowrap; text-align: right">±260.99%</td>
    <td style="white-space: nowrap; text-align: right">453.50 ns</td>
    <td style="white-space: nowrap; text-align: right">16737 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.0456 M</td>
    <td style="white-space: nowrap; text-align: right">21952.10 ns</td>
    <td style="white-space: nowrap; text-align: right">±4.14%</td>
    <td style="white-space: nowrap; text-align: right">21706 ns</td>
    <td style="white-space: nowrap; text-align: right">23547 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">list ++ [val] (list)</td>
    <td style="white-space: nowrap; text-align: right">0.0130 M</td>
    <td style="white-space: nowrap; text-align: right">76696.02 ns</td>
    <td style="white-space: nowrap; text-align: right">±70.31%</td>
    <td style="white-space: nowrap; text-align: right">103114 ns</td>
    <td style="white-space: nowrap; text-align: right">282351.40 ns</td>
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
    <td style="white-space: nowrap;text-align: right">3.10 M</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">1.54 M</td>
    <td style="white-space: nowrap; text-align: right">2.01x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">1.50 M</td>
    <td style="white-space: nowrap; text-align: right">2.07x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.0456 M</td>
    <td style="white-space: nowrap; text-align: right">68.04x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">list ++ [val] (list)</td>
    <td style="white-space: nowrap; text-align: right">0.0130 M</td>
    <td style="white-space: nowrap; text-align: right">237.71x</td>
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
    <td style="white-space: nowrap; text-align: right">1539.59 K</td>
    <td style="white-space: nowrap; text-align: right">0.65 μs</td>
    <td style="white-space: nowrap; text-align: right">±106.96%</td>
    <td style="white-space: nowrap; text-align: right">0.50 μs</td>
    <td style="white-space: nowrap; text-align: right">5.38 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">1003.65 K</td>
    <td style="white-space: nowrap; text-align: right">1.00 μs</td>
    <td style="white-space: nowrap; text-align: right">±15.88%</td>
    <td style="white-space: nowrap; text-align: right">0.94 μs</td>
    <td style="white-space: nowrap; text-align: right">1.49 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">754.40 K</td>
    <td style="white-space: nowrap; text-align: right">1.33 μs</td>
    <td style="white-space: nowrap; text-align: right">±179.34%</td>
    <td style="white-space: nowrap; text-align: right">0.87 μs</td>
    <td style="white-space: nowrap; text-align: right">15.49 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">43.60 K</td>
    <td style="white-space: nowrap; text-align: right">22.93 μs</td>
    <td style="white-space: nowrap; text-align: right">±5.84%</td>
    <td style="white-space: nowrap; text-align: right">23.28 μs</td>
    <td style="white-space: nowrap; text-align: right">24.75 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">list ++ [val] (list)</td>
    <td style="white-space: nowrap; text-align: right">4.40 K</td>
    <td style="white-space: nowrap; text-align: right">227.17 μs</td>
    <td style="white-space: nowrap; text-align: right">±66.09%</td>
    <td style="white-space: nowrap; text-align: right">307.47 μs</td>
    <td style="white-space: nowrap; text-align: right">532.45 μs</td>
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
    <td style="white-space: nowrap;text-align: right">1539.59 K</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">1003.65 K</td>
    <td style="white-space: nowrap; text-align: right">1.53x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">754.40 K</td>
    <td style="white-space: nowrap; text-align: right">2.04x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">43.60 K</td>
    <td style="white-space: nowrap; text-align: right">35.31x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">list ++ [val] (list)</td>
    <td style="white-space: nowrap; text-align: right">4.40 K</td>
    <td style="white-space: nowrap; text-align: right">349.74x</td>
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
    <td style="white-space: nowrap">520 B</td>
    <td>32.5x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (ErlangArray)</td>
    <td style="white-space: nowrap">560 B</td>
    <td>35.0x</td>
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
    <td style="white-space: nowrap; text-align: right">1200.22 K</td>
    <td style="white-space: nowrap; text-align: right">0.83 μs</td>
    <td style="white-space: nowrap; text-align: right">±24.36%</td>
    <td style="white-space: nowrap; text-align: right">0.80 μs</td>
    <td style="white-space: nowrap; text-align: right">1.74 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">1032.68 K</td>
    <td style="white-space: nowrap; text-align: right">0.97 μs</td>
    <td style="white-space: nowrap; text-align: right">±13.83%</td>
    <td style="white-space: nowrap; text-align: right">0.93 μs</td>
    <td style="white-space: nowrap; text-align: right">1.36 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">923.41 K</td>
    <td style="white-space: nowrap; text-align: right">1.08 μs</td>
    <td style="white-space: nowrap; text-align: right">±9.80%</td>
    <td style="white-space: nowrap; text-align: right">1.04 μs</td>
    <td style="white-space: nowrap; text-align: right">1.41 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">41.63 K</td>
    <td style="white-space: nowrap; text-align: right">24.02 μs</td>
    <td style="white-space: nowrap; text-align: right">±6.30%</td>
    <td style="white-space: nowrap; text-align: right">24.51 μs</td>
    <td style="white-space: nowrap; text-align: right">25.23 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">list ++ [val] (list)</td>
    <td style="white-space: nowrap; text-align: right">3.22 K</td>
    <td style="white-space: nowrap; text-align: right">310.09 μs</td>
    <td style="white-space: nowrap; text-align: right">±75.41%</td>
    <td style="white-space: nowrap; text-align: right">118.94 μs</td>
    <td style="white-space: nowrap; text-align: right">609.19 μs</td>
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
    <td style="white-space: nowrap;text-align: right">1200.22 K</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">1032.68 K</td>
    <td style="white-space: nowrap; text-align: right">1.16x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">923.41 K</td>
    <td style="white-space: nowrap; text-align: right">1.3x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">41.63 K</td>
    <td style="white-space: nowrap; text-align: right">28.83x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">list ++ [val] (list)</td>
    <td style="white-space: nowrap; text-align: right">3.22 K</td>
    <td style="white-space: nowrap; text-align: right">372.18x</td>
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
    <td style="white-space: nowrap">552 B</td>
    <td>34.5x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (ErlangArray)</td>
    <td style="white-space: nowrap">560 B</td>
    <td>35.0x</td>
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
    <td style="white-space: nowrap; text-align: right">1172.56 K</td>
    <td style="white-space: nowrap; text-align: right">0.85 μs</td>
    <td style="white-space: nowrap; text-align: right">±21.14%</td>
    <td style="white-space: nowrap; text-align: right">0.76 μs</td>
    <td style="white-space: nowrap; text-align: right">1.24 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">890.08 K</td>
    <td style="white-space: nowrap; text-align: right">1.12 μs</td>
    <td style="white-space: nowrap; text-align: right">±5.91%</td>
    <td style="white-space: nowrap; text-align: right">1.12 μs</td>
    <td style="white-space: nowrap; text-align: right">1.27 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">524.34 K</td>
    <td style="white-space: nowrap; text-align: right">1.91 μs</td>
    <td style="white-space: nowrap; text-align: right">±88.92%</td>
    <td style="white-space: nowrap; text-align: right">1.18 μs</td>
    <td style="white-space: nowrap; text-align: right">5.73 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">42.28 K</td>
    <td style="white-space: nowrap; text-align: right">23.65 μs</td>
    <td style="white-space: nowrap; text-align: right">±0.59%</td>
    <td style="white-space: nowrap; text-align: right">23.65 μs</td>
    <td style="white-space: nowrap; text-align: right">23.75 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">list ++ [val] (list)</td>
    <td style="white-space: nowrap; text-align: right">2.76 K</td>
    <td style="white-space: nowrap; text-align: right">362.85 μs</td>
    <td style="white-space: nowrap; text-align: right">±65.04%</td>
    <td style="white-space: nowrap; text-align: right">268.06 μs</td>
    <td style="white-space: nowrap; text-align: right">900.21 μs</td>
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
    <td style="white-space: nowrap;text-align: right">1172.56 K</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">890.08 K</td>
    <td style="white-space: nowrap; text-align: right">1.32x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">524.34 K</td>
    <td style="white-space: nowrap; text-align: right">2.24x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">42.28 K</td>
    <td style="white-space: nowrap; text-align: right">27.74x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">list ++ [val] (list)</td>
    <td style="white-space: nowrap; text-align: right">2.76 K</td>
    <td style="white-space: nowrap; text-align: right">425.46x</td>
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
    <td style="white-space: nowrap; text-align: right">1057.31 K</td>
    <td style="white-space: nowrap; text-align: right">0.95 μs</td>
    <td style="white-space: nowrap; text-align: right">±19.72%</td>
    <td style="white-space: nowrap; text-align: right">0.87 μs</td>
    <td style="white-space: nowrap; text-align: right">1.20 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">800.43 K</td>
    <td style="white-space: nowrap; text-align: right">1.25 μs</td>
    <td style="white-space: nowrap; text-align: right">±10.09%</td>
    <td style="white-space: nowrap; text-align: right">1.25 μs</td>
    <td style="white-space: nowrap; text-align: right">1.37 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">791.30 K</td>
    <td style="white-space: nowrap; text-align: right">1.26 μs</td>
    <td style="white-space: nowrap; text-align: right">±7.68%</td>
    <td style="white-space: nowrap; text-align: right">1.26 μs</td>
    <td style="white-space: nowrap; text-align: right">1.36 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">38.23 K</td>
    <td style="white-space: nowrap; text-align: right">26.16 μs</td>
    <td style="white-space: nowrap; text-align: right">±0.00%</td>
    <td style="white-space: nowrap; text-align: right">26.16 μs</td>
    <td style="white-space: nowrap; text-align: right">26.16 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">list ++ [val] (list)</td>
    <td style="white-space: nowrap; text-align: right">1.74 K</td>
    <td style="white-space: nowrap; text-align: right">573.20 μs</td>
    <td style="white-space: nowrap; text-align: right">±5.82%</td>
    <td style="white-space: nowrap; text-align: right">560.12 μs</td>
    <td style="white-space: nowrap; text-align: right">631.25 μs</td>
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
    <td style="white-space: nowrap;text-align: right">1057.31 K</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">800.43 K</td>
    <td style="white-space: nowrap; text-align: right">1.32x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">791.30 K</td>
    <td style="white-space: nowrap; text-align: right">1.34x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">38.23 K</td>
    <td style="white-space: nowrap; text-align: right">27.66x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">list ++ [val] (list)</td>
    <td style="white-space: nowrap; text-align: right">1.74 K</td>
    <td style="white-space: nowrap; text-align: right">606.04x</td>
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
    <td style="white-space: nowrap">648 B</td>
    <td>40.5x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (ErlangArray)</td>
    <td style="white-space: nowrap">656 B</td>
    <td>41.0x</td>
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
    <td style="white-space: nowrap; text-align: right">879.89 K</td>
    <td style="white-space: nowrap; text-align: right">1.14 μs</td>
    <td style="white-space: nowrap; text-align: right">±3.80%</td>
    <td style="white-space: nowrap; text-align: right">1.14 μs</td>
    <td style="white-space: nowrap; text-align: right">1.17 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">[val | list] (list, backwards)</td>
    <td style="white-space: nowrap; text-align: right">872.22 K</td>
    <td style="white-space: nowrap; text-align: right">1.15 μs</td>
    <td style="white-space: nowrap; text-align: right">±5.49%</td>
    <td style="white-space: nowrap; text-align: right">1.15 μs</td>
    <td style="white-space: nowrap; text-align: right">1.19 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">637.35 K</td>
    <td style="white-space: nowrap; text-align: right">1.57 μs</td>
    <td style="white-space: nowrap; text-align: right">±1.08%</td>
    <td style="white-space: nowrap; text-align: right">1.57 μs</td>
    <td style="white-space: nowrap; text-align: right">1.58 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">33.13 K</td>
    <td style="white-space: nowrap; text-align: right">30.18 μs</td>
    <td style="white-space: nowrap; text-align: right">±0.00%</td>
    <td style="white-space: nowrap; text-align: right">30.18 μs</td>
    <td style="white-space: nowrap; text-align: right">30.18 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">list ++ [val] (list)</td>
    <td style="white-space: nowrap; text-align: right">0.22 K</td>
    <td style="white-space: nowrap; text-align: right">4546.67 μs</td>
    <td style="white-space: nowrap; text-align: right">±17.80%</td>
    <td style="white-space: nowrap; text-align: right">4546.67 μs</td>
    <td style="white-space: nowrap; text-align: right">5118.88 μs</td>
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
    <td style="white-space: nowrap;text-align: right">879.89 K</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">[val | list] (list, backwards)</td>
    <td style="white-space: nowrap; text-align: right">872.22 K</td>
    <td style="white-space: nowrap; text-align: right">1.01x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">637.35 K</td>
    <td style="white-space: nowrap; text-align: right">1.38x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">33.13 K</td>
    <td style="white-space: nowrap; text-align: right">26.56x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">list ++ [val] (list)</td>
    <td style="white-space: nowrap; text-align: right">0.22 K</td>
    <td style="white-space: nowrap; text-align: right">4000.59x</td>
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
    <td style="white-space: nowrap; text-align: right">892.86 K</td>
    <td style="white-space: nowrap; text-align: right">1.12 μs</td>
    <td style="white-space: nowrap; text-align: right">±0.00%</td>
    <td style="white-space: nowrap; text-align: right">1.12 μs</td>
    <td style="white-space: nowrap; text-align: right">1.12 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">[val | list] (list, backwards)</td>
    <td style="white-space: nowrap; text-align: right">848.90 K</td>
    <td style="white-space: nowrap; text-align: right">1.18 μs</td>
    <td style="white-space: nowrap; text-align: right">±0.00%</td>
    <td style="white-space: nowrap; text-align: right">1.18 μs</td>
    <td style="white-space: nowrap; text-align: right">1.18 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">733.14 K</td>
    <td style="white-space: nowrap; text-align: right">1.36 μs</td>
    <td style="white-space: nowrap; text-align: right">±0.00%</td>
    <td style="white-space: nowrap; text-align: right">1.36 μs</td>
    <td style="white-space: nowrap; text-align: right">1.36 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">19.31 K</td>
    <td style="white-space: nowrap; text-align: right">51.79 μs</td>
    <td style="white-space: nowrap; text-align: right">±0.00%</td>
    <td style="white-space: nowrap; text-align: right">51.79 μs</td>
    <td style="white-space: nowrap; text-align: right">51.79 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">list ++ [val] (list)</td>
    <td style="white-space: nowrap; text-align: right">0.136 K</td>
    <td style="white-space: nowrap; text-align: right">7347.11 μs</td>
    <td style="white-space: nowrap; text-align: right">±0.00%</td>
    <td style="white-space: nowrap; text-align: right">7347.11 μs</td>
    <td style="white-space: nowrap; text-align: right">7347.11 μs</td>
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
    <td style="white-space: nowrap;text-align: right">892.86 K</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">[val | list] (list, backwards)</td>
    <td style="white-space: nowrap; text-align: right">848.90 K</td>
    <td style="white-space: nowrap; text-align: right">1.05x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">733.14 K</td>
    <td style="white-space: nowrap; text-align: right">1.22x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">19.31 K</td>
    <td style="white-space: nowrap; text-align: right">46.24x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">list ++ [val] (list)</td>
    <td style="white-space: nowrap; text-align: right">0.136 K</td>
    <td style="white-space: nowrap; text-align: right">6559.92x</td>
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
    <td style="white-space: nowrap">Arrays.append/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">938.97 K</td>
    <td style="white-space: nowrap; text-align: right">1.06 μs</td>
    <td style="white-space: nowrap; text-align: right">±0.00%</td>
    <td style="white-space: nowrap; text-align: right">1.06 μs</td>
    <td style="white-space: nowrap; text-align: right">1.06 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">[val | list] (list, backwards)</td>
    <td style="white-space: nowrap; text-align: right">894.45 K</td>
    <td style="white-space: nowrap; text-align: right">1.12 μs</td>
    <td style="white-space: nowrap; text-align: right">±0.00%</td>
    <td style="white-space: nowrap; text-align: right">1.12 μs</td>
    <td style="white-space: nowrap; text-align: right">1.12 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">565.29 K</td>
    <td style="white-space: nowrap; text-align: right">1.77 μs</td>
    <td style="white-space: nowrap; text-align: right">±0.00%</td>
    <td style="white-space: nowrap; text-align: right">1.77 μs</td>
    <td style="white-space: nowrap; text-align: right">1.77 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">15.38 K</td>
    <td style="white-space: nowrap; text-align: right">65.00 μs</td>
    <td style="white-space: nowrap; text-align: right">±0.00%</td>
    <td style="white-space: nowrap; text-align: right">65.00 μs</td>
    <td style="white-space: nowrap; text-align: right">65.00 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">list ++ [val] (list)</td>
    <td style="white-space: nowrap; text-align: right">0.197 K</td>
    <td style="white-space: nowrap; text-align: right">5082.44 μs</td>
    <td style="white-space: nowrap; text-align: right">±0.00%</td>
    <td style="white-space: nowrap; text-align: right">5082.44 μs</td>
    <td style="white-space: nowrap; text-align: right">5082.44 μs</td>
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
    <td style="white-space: nowrap;text-align: right">938.97 K</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">[val | list] (list, backwards)</td>
    <td style="white-space: nowrap; text-align: right">894.45 K</td>
    <td style="white-space: nowrap; text-align: right">1.05x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">565.29 K</td>
    <td style="white-space: nowrap; text-align: right">1.66x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">15.38 K</td>
    <td style="white-space: nowrap; text-align: right">61.04x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">list ++ [val] (list)</td>
    <td style="white-space: nowrap; text-align: right">0.197 K</td>
    <td style="white-space: nowrap; text-align: right">4772.24x</td>
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
    <td style="white-space: nowrap">752 B</td>
<td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">[val | list] (list, backwards)</td>
    <td style="white-space: nowrap">16 B</td>
    <td>0.02x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (MapArray)</td>
    <td style="white-space: nowrap">704 B</td>
    <td>0.94x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.append/2 (RRBVector)</td>
    <td style="white-space: nowrap">88 B</td>
    <td>0.12x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">list ++ [val] (list)</td>
    <td style="white-space: nowrap">3121584 B</td>
    <td>4151.04x</td>
  </tr>

</table>


<hr/>

