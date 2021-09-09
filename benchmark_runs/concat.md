
# Benchmark

Comparing `Arrays.concat` with `Kernel.++`,
by concatenating two collections of the same size.


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
    <td style="white-space: nowrap">Kernel.++/2 (list)</td>
    <td style="white-space: nowrap; text-align: right">3886.07 K</td>
    <td style="white-space: nowrap; text-align: right">0.26 μs</td>
    <td style="white-space: nowrap; text-align: right">±778.28%</td>
    <td style="white-space: nowrap; text-align: right">0.104 μs</td>
    <td style="white-space: nowrap; text-align: right">1.43 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">95.36 K</td>
    <td style="white-space: nowrap; text-align: right">10.49 μs</td>
    <td style="white-space: nowrap; text-align: right">±81.25%</td>
    <td style="white-space: nowrap; text-align: right">7.47 μs</td>
    <td style="white-space: nowrap; text-align: right">36.40 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">65.23 K</td>
    <td style="white-space: nowrap; text-align: right">15.33 μs</td>
    <td style="white-space: nowrap; text-align: right">±62.52%</td>
    <td style="white-space: nowrap; text-align: right">11.14 μs</td>
    <td style="white-space: nowrap; text-align: right">47.13 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">24.91 K</td>
    <td style="white-space: nowrap; text-align: right">40.15 μs</td>
    <td style="white-space: nowrap; text-align: right">±50.92%</td>
    <td style="white-space: nowrap; text-align: right">29.03 μs</td>
    <td style="white-space: nowrap; text-align: right">117.49 μs</td>
  </tr>

</table>


Comparison

<table style="width: 1%">
  <tr>
    <th>Name</th>
    <th style="text-align: right">IPS</th>
    <th style="text-align: right">Slower</th>
  <tr>
    <td style="white-space: nowrap">Kernel.++/2 (list)</td>
    <td style="white-space: nowrap;text-align: right">3886.07 K</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">95.36 K</td>
    <td style="white-space: nowrap; text-align: right">40.75x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">65.23 K</td>
    <td style="white-space: nowrap; text-align: right">59.57x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">24.91 K</td>
    <td style="white-space: nowrap; text-align: right">156.01x</td>
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
    <td style="white-space: nowrap">Kernel.++/2 (list)</td>
    <td style="white-space: nowrap">0.47 KB</td>
<td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (ErlangArray)</td>
    <td style="white-space: nowrap">10.21 KB</td>
    <td>21.78x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (MapArray)</td>
    <td style="white-space: nowrap">10.09 KB</td>
    <td>21.53x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (RRBVector)</td>
    <td style="white-space: nowrap">8.18 KB</td>
    <td>17.45x</td>
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
    <td style="white-space: nowrap">Kernel.++/2 (list)</td>
    <td style="white-space: nowrap; text-align: right">2099.04 K</td>
    <td style="white-space: nowrap; text-align: right">0.48 μs</td>
    <td style="white-space: nowrap; text-align: right">±546.09%</td>
    <td style="white-space: nowrap; text-align: right">0.196 μs</td>
    <td style="white-space: nowrap; text-align: right">15.34 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">50.07 K</td>
    <td style="white-space: nowrap; text-align: right">19.97 μs</td>
    <td style="white-space: nowrap; text-align: right">±41.61%</td>
    <td style="white-space: nowrap; text-align: right">17.38 μs</td>
    <td style="white-space: nowrap; text-align: right">55.60 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">45.16 K</td>
    <td style="white-space: nowrap; text-align: right">22.14 μs</td>
    <td style="white-space: nowrap; text-align: right">±35.68%</td>
    <td style="white-space: nowrap; text-align: right">20.39 μs</td>
    <td style="white-space: nowrap; text-align: right">57.59 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">12.36 K</td>
    <td style="white-space: nowrap; text-align: right">80.92 μs</td>
    <td style="white-space: nowrap; text-align: right">±37.20%</td>
    <td style="white-space: nowrap; text-align: right">70.83 μs</td>
    <td style="white-space: nowrap; text-align: right">180.25 μs</td>
  </tr>

</table>


Comparison

<table style="width: 1%">
  <tr>
    <th>Name</th>
    <th style="text-align: right">IPS</th>
    <th style="text-align: right">Slower</th>
  <tr>
    <td style="white-space: nowrap">Kernel.++/2 (list)</td>
    <td style="white-space: nowrap;text-align: right">2099.04 K</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">50.07 K</td>
    <td style="white-space: nowrap; text-align: right">41.92x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">45.16 K</td>
    <td style="white-space: nowrap; text-align: right">46.48x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">12.36 K</td>
    <td style="white-space: nowrap; text-align: right">169.86x</td>
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
    <td style="white-space: nowrap">Kernel.++/2 (list)</td>
    <td style="white-space: nowrap">0.58 KB</td>
<td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (ErlangArray)</td>
    <td style="white-space: nowrap">23.49 KB</td>
    <td>40.64x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (MapArray)</td>
    <td style="white-space: nowrap">18.52 KB</td>
    <td>32.03x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (RRBVector)</td>
    <td style="white-space: nowrap">16.18 KB</td>
    <td>27.99x</td>
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
    <td style="white-space: nowrap">Kernel.++/2 (list)</td>
    <td style="white-space: nowrap; text-align: right">1091.93 K</td>
    <td style="white-space: nowrap; text-align: right">0.92 μs</td>
    <td style="white-space: nowrap; text-align: right">±340.64%</td>
    <td style="white-space: nowrap; text-align: right">0.38 μs</td>
    <td style="white-space: nowrap; text-align: right">22.08 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">22.63 K</td>
    <td style="white-space: nowrap; text-align: right">44.19 μs</td>
    <td style="white-space: nowrap; text-align: right">±27.49%</td>
    <td style="white-space: nowrap; text-align: right">41.11 μs</td>
    <td style="white-space: nowrap; text-align: right">104.59 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">20.29 K</td>
    <td style="white-space: nowrap; text-align: right">49.29 μs</td>
    <td style="white-space: nowrap; text-align: right">±30.72%</td>
    <td style="white-space: nowrap; text-align: right">46.97 μs</td>
    <td style="white-space: nowrap; text-align: right">122.34 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">6.09 K</td>
    <td style="white-space: nowrap; text-align: right">164.24 μs</td>
    <td style="white-space: nowrap; text-align: right">±27.97%</td>
    <td style="white-space: nowrap; text-align: right">162.53 μs</td>
    <td style="white-space: nowrap; text-align: right">343.23 μs</td>
  </tr>

</table>


Comparison

<table style="width: 1%">
  <tr>
    <th>Name</th>
    <th style="text-align: right">IPS</th>
    <th style="text-align: right">Slower</th>
  <tr>
    <td style="white-space: nowrap">Kernel.++/2 (list)</td>
    <td style="white-space: nowrap;text-align: right">1091.93 K</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">22.63 K</td>
    <td style="white-space: nowrap; text-align: right">48.26x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">20.29 K</td>
    <td style="white-space: nowrap; text-align: right">53.82x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">6.09 K</td>
    <td style="white-space: nowrap; text-align: right">179.34x</td>
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
    <td style="white-space: nowrap">Kernel.++/2 (list)</td>
    <td style="white-space: nowrap">0.41 KB</td>
<td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (ErlangArray)</td>
    <td style="white-space: nowrap">53.13 KB</td>
    <td>130.79x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (MapArray)</td>
    <td style="white-space: nowrap">41.41 KB</td>
    <td>101.94x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (RRBVector)</td>
    <td style="white-space: nowrap">32.18 KB</td>
    <td>79.21x</td>
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
    <td style="white-space: nowrap">Kernel.++/2 (list)</td>
    <td style="white-space: nowrap; text-align: right">735.67 K</td>
    <td style="white-space: nowrap; text-align: right">1.36 μs</td>
    <td style="white-space: nowrap; text-align: right">±175.89%</td>
    <td style="white-space: nowrap; text-align: right">0.71 μs</td>
    <td style="white-space: nowrap; text-align: right">10.60 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">11.56 K</td>
    <td style="white-space: nowrap; text-align: right">86.53 μs</td>
    <td style="white-space: nowrap; text-align: right">±22.67%</td>
    <td style="white-space: nowrap; text-align: right">81.13 μs</td>
    <td style="white-space: nowrap; text-align: right">182.81 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">8.58 K</td>
    <td style="white-space: nowrap; text-align: right">116.52 μs</td>
    <td style="white-space: nowrap; text-align: right">±24.36%</td>
    <td style="white-space: nowrap; text-align: right">109.49 μs</td>
    <td style="white-space: nowrap; text-align: right">260.58 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">2.59 K</td>
    <td style="white-space: nowrap; text-align: right">386.81 μs</td>
    <td style="white-space: nowrap; text-align: right">±24.31%</td>
    <td style="white-space: nowrap; text-align: right">373.17 μs</td>
    <td style="white-space: nowrap; text-align: right">758.80 μs</td>
  </tr>

</table>


Comparison

<table style="width: 1%">
  <tr>
    <th>Name</th>
    <th style="text-align: right">IPS</th>
    <th style="text-align: right">Slower</th>
  <tr>
    <td style="white-space: nowrap">Kernel.++/2 (list)</td>
    <td style="white-space: nowrap;text-align: right">735.67 K</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">11.56 K</td>
    <td style="white-space: nowrap; text-align: right">63.66x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">8.58 K</td>
    <td style="white-space: nowrap; text-align: right">85.72x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">2.59 K</td>
    <td style="white-space: nowrap; text-align: right">284.56x</td>
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
    <td style="white-space: nowrap">Kernel.++/2 (list)</td>
    <td style="white-space: nowrap">4 KB</td>
<td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (ErlangArray)</td>
    <td style="white-space: nowrap">106.27 KB</td>
    <td>26.57x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (MapArray)</td>
    <td style="white-space: nowrap">94.52 KB</td>
    <td>23.63x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (RRBVector)</td>
    <td style="white-space: nowrap">64.18 KB</td>
    <td>16.04x</td>
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
    <td style="white-space: nowrap">Kernel.++/2 (list)</td>
    <td style="white-space: nowrap; text-align: right">472.16 K</td>
    <td style="white-space: nowrap; text-align: right">2.12 μs</td>
    <td style="white-space: nowrap; text-align: right">±108.31%</td>
    <td style="white-space: nowrap; text-align: right">1.54 μs</td>
    <td style="white-space: nowrap; text-align: right">9.85 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">5.76 K</td>
    <td style="white-space: nowrap; text-align: right">173.63 μs</td>
    <td style="white-space: nowrap; text-align: right">±22.87%</td>
    <td style="white-space: nowrap; text-align: right">164.02 μs</td>
    <td style="white-space: nowrap; text-align: right">369.48 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">3.74 K</td>
    <td style="white-space: nowrap; text-align: right">267.63 μs</td>
    <td style="white-space: nowrap; text-align: right">±22.28%</td>
    <td style="white-space: nowrap; text-align: right">252.65 μs</td>
    <td style="white-space: nowrap; text-align: right">558.71 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">1.21 K</td>
    <td style="white-space: nowrap; text-align: right">824.82 μs</td>
    <td style="white-space: nowrap; text-align: right">±15.12%</td>
    <td style="white-space: nowrap; text-align: right">797.62 μs</td>
    <td style="white-space: nowrap; text-align: right">1271.69 μs</td>
  </tr>

</table>


Comparison

<table style="width: 1%">
  <tr>
    <th>Name</th>
    <th style="text-align: right">IPS</th>
    <th style="text-align: right">Slower</th>
  <tr>
    <td style="white-space: nowrap">Kernel.++/2 (list)</td>
    <td style="white-space: nowrap;text-align: right">472.16 K</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">5.76 K</td>
    <td style="white-space: nowrap; text-align: right">81.98x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">3.74 K</td>
    <td style="white-space: nowrap; text-align: right">126.36x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">1.21 K</td>
    <td style="white-space: nowrap; text-align: right">389.44x</td>
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
    <td style="white-space: nowrap">Kernel.++/2 (list)</td>
    <td style="white-space: nowrap">3.84 KB</td>
<td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (ErlangArray)</td>
    <td style="white-space: nowrap">214.63 KB</td>
    <td>55.84x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (MapArray)</td>
    <td style="white-space: nowrap">206.27 KB</td>
    <td>53.66x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (RRBVector)</td>
    <td style="white-space: nowrap">128.18 KB</td>
    <td>33.35x</td>
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
    <td style="white-space: nowrap">Kernel.++/2 (list)</td>
    <td style="white-space: nowrap; text-align: right">107.57 K</td>
    <td style="white-space: nowrap; text-align: right">9.30 μs</td>
    <td style="white-space: nowrap; text-align: right">±111.25%</td>
    <td style="white-space: nowrap; text-align: right">3.53 μs</td>
    <td style="white-space: nowrap; text-align: right">41.17 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">2.49 K</td>
    <td style="white-space: nowrap; text-align: right">402.02 μs</td>
    <td style="white-space: nowrap; text-align: right">±15.58%</td>
    <td style="white-space: nowrap; text-align: right">384.48 μs</td>
    <td style="white-space: nowrap; text-align: right">701.70 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">1.88 K</td>
    <td style="white-space: nowrap; text-align: right">533.00 μs</td>
    <td style="white-space: nowrap; text-align: right">±17.73%</td>
    <td style="white-space: nowrap; text-align: right">510.58 μs</td>
    <td style="white-space: nowrap; text-align: right">960.45 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.61 K</td>
    <td style="white-space: nowrap; text-align: right">1635.64 μs</td>
    <td style="white-space: nowrap; text-align: right">±27.46%</td>
    <td style="white-space: nowrap; text-align: right">1536.64 μs</td>
    <td style="white-space: nowrap; text-align: right">5380.70 μs</td>
  </tr>

</table>


Comparison

<table style="width: 1%">
  <tr>
    <th>Name</th>
    <th style="text-align: right">IPS</th>
    <th style="text-align: right">Slower</th>
  <tr>
    <td style="white-space: nowrap">Kernel.++/2 (list)</td>
    <td style="white-space: nowrap;text-align: right">107.57 K</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">2.49 K</td>
    <td style="white-space: nowrap; text-align: right">43.25x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">1.88 K</td>
    <td style="white-space: nowrap; text-align: right">57.34x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.61 K</td>
    <td style="white-space: nowrap; text-align: right">175.95x</td>
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
    <td style="white-space: nowrap">Kernel.++/2 (list)</td>
    <td style="white-space: nowrap">0.34 KB</td>
<td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (ErlangArray)</td>
    <td style="white-space: nowrap">520.64 KB</td>
    <td>1514.59x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (MapArray)</td>
    <td style="white-space: nowrap">440.95 KB</td>
    <td>1282.75x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (RRBVector)</td>
    <td style="white-space: nowrap">256.18 KB</td>
    <td>745.25x</td>
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
    <td style="white-space: nowrap">Kernel.++/2 (list)</td>
    <td style="white-space: nowrap; text-align: right">65.24 K</td>
    <td style="white-space: nowrap; text-align: right">0.0153 ms</td>
    <td style="white-space: nowrap; text-align: right">±90.67%</td>
    <td style="white-space: nowrap; text-align: right">0.00675 ms</td>
    <td style="white-space: nowrap; text-align: right">0.0707 ms</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">1.25 K</td>
    <td style="white-space: nowrap; text-align: right">0.80 ms</td>
    <td style="white-space: nowrap; text-align: right">±15.44%</td>
    <td style="white-space: nowrap; text-align: right">0.75 ms</td>
    <td style="white-space: nowrap; text-align: right">1.26 ms</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">0.93 K</td>
    <td style="white-space: nowrap; text-align: right">1.08 ms</td>
    <td style="white-space: nowrap; text-align: right">±14.14%</td>
    <td style="white-space: nowrap; text-align: right">1.05 ms</td>
    <td style="white-space: nowrap; text-align: right">1.57 ms</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.29 K</td>
    <td style="white-space: nowrap; text-align: right">3.41 ms</td>
    <td style="white-space: nowrap; text-align: right">±7.95%</td>
    <td style="white-space: nowrap; text-align: right">3.44 ms</td>
    <td style="white-space: nowrap; text-align: right">4.25 ms</td>
  </tr>

</table>


Comparison

<table style="width: 1%">
  <tr>
    <th>Name</th>
    <th style="text-align: right">IPS</th>
    <th style="text-align: right">Slower</th>
  <tr>
    <td style="white-space: nowrap">Kernel.++/2 (list)</td>
    <td style="white-space: nowrap;text-align: right">65.24 K</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">1.25 K</td>
    <td style="white-space: nowrap; text-align: right">52.25x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">0.93 K</td>
    <td style="white-space: nowrap; text-align: right">70.38x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.29 K</td>
    <td style="white-space: nowrap; text-align: right">222.45x</td>
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
    <td style="white-space: nowrap">Kernel.++/2 (list)</td>
    <td style="white-space: nowrap">21.25 KB</td>
<td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (ErlangArray)</td>
    <td style="white-space: nowrap">1040.95 KB</td>
    <td>48.99x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (MapArray)</td>
    <td style="white-space: nowrap">943.59 KB</td>
    <td>44.4x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (RRBVector)</td>
    <td style="white-space: nowrap">512.18 KB</td>
    <td>24.1x</td>
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
    <td style="white-space: nowrap">Kernel.++/2 (list)</td>
    <td style="white-space: nowrap; text-align: right">28174.94</td>
    <td style="white-space: nowrap; text-align: right">0.0355 ms</td>
    <td style="white-space: nowrap; text-align: right">±69.79%</td>
    <td style="white-space: nowrap; text-align: right">0.0494 ms</td>
    <td style="white-space: nowrap; text-align: right">0.120 ms</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">618.88</td>
    <td style="white-space: nowrap; text-align: right">1.62 ms</td>
    <td style="white-space: nowrap; text-align: right">±11.86%</td>
    <td style="white-space: nowrap; text-align: right">1.54 ms</td>
    <td style="white-space: nowrap; text-align: right">2.15 ms</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">451.39</td>
    <td style="white-space: nowrap; text-align: right">2.22 ms</td>
    <td style="white-space: nowrap; text-align: right">±8.16%</td>
    <td style="white-space: nowrap; text-align: right">2.16 ms</td>
    <td style="white-space: nowrap; text-align: right">2.60 ms</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">155.04</td>
    <td style="white-space: nowrap; text-align: right">6.45 ms</td>
    <td style="white-space: nowrap; text-align: right">±6.31%</td>
    <td style="white-space: nowrap; text-align: right">6.46 ms</td>
    <td style="white-space: nowrap; text-align: right">7.17 ms</td>
  </tr>

</table>


Comparison

<table style="width: 1%">
  <tr>
    <th>Name</th>
    <th style="text-align: right">IPS</th>
    <th style="text-align: right">Slower</th>
  <tr>
    <td style="white-space: nowrap">Kernel.++/2 (list)</td>
    <td style="white-space: nowrap;text-align: right">28174.94</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">618.88</td>
    <td style="white-space: nowrap; text-align: right">45.53x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">451.39</td>
    <td style="white-space: nowrap; text-align: right">62.42x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">155.04</td>
    <td style="white-space: nowrap; text-align: right">181.72x</td>
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
    <td style="white-space: nowrap">Kernel.++/2 (list)</td>
    <td style="white-space: nowrap">0.00993 MB</td>
<td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (ErlangArray)</td>
    <td style="white-space: nowrap">2.03 MB</td>
    <td>204.81x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (MapArray)</td>
    <td style="white-space: nowrap">2.00 MB</td>
    <td>200.92x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (RRBVector)</td>
    <td style="white-space: nowrap">1.00 MB</td>
    <td>100.69x</td>
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
    <td style="white-space: nowrap">Kernel.++/2 (list)</td>
    <td style="white-space: nowrap; text-align: right">14527.29</td>
    <td style="white-space: nowrap; text-align: right">0.0688 ms</td>
    <td style="white-space: nowrap; text-align: right">±69.23%</td>
    <td style="white-space: nowrap; text-align: right">0.0333 ms</td>
    <td style="white-space: nowrap; text-align: right">0.21 ms</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">278.97</td>
    <td style="white-space: nowrap; text-align: right">3.58 ms</td>
    <td style="white-space: nowrap; text-align: right">±9.32%</td>
    <td style="white-space: nowrap; text-align: right">3.47 ms</td>
    <td style="white-space: nowrap; text-align: right">4.30 ms</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">209.43</td>
    <td style="white-space: nowrap; text-align: right">4.77 ms</td>
    <td style="white-space: nowrap; text-align: right">±5.55%</td>
    <td style="white-space: nowrap; text-align: right">4.80 ms</td>
    <td style="white-space: nowrap; text-align: right">5.37 ms</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">75.14</td>
    <td style="white-space: nowrap; text-align: right">13.31 ms</td>
    <td style="white-space: nowrap; text-align: right">±4.08%</td>
    <td style="white-space: nowrap; text-align: right">13.21 ms</td>
    <td style="white-space: nowrap; text-align: right">14.81 ms</td>
  </tr>

</table>


Comparison

<table style="width: 1%">
  <tr>
    <th>Name</th>
    <th style="text-align: right">IPS</th>
    <th style="text-align: right">Slower</th>
  <tr>
    <td style="white-space: nowrap">Kernel.++/2 (list)</td>
    <td style="white-space: nowrap;text-align: right">14527.29</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">278.97</td>
    <td style="white-space: nowrap; text-align: right">52.08x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">209.43</td>
    <td style="white-space: nowrap; text-align: right">69.37x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">75.14</td>
    <td style="white-space: nowrap; text-align: right">193.33x</td>
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
    <td style="white-space: nowrap">Kernel.++/2 (list)</td>
    <td style="white-space: nowrap">0.104 MB</td>
<td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (ErlangArray)</td>
    <td style="white-space: nowrap">4.66 MB</td>
    <td>44.88x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (MapArray)</td>
    <td style="white-space: nowrap">4.28 MB</td>
    <td>41.25x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (RRBVector)</td>
    <td style="white-space: nowrap">2.00 MB</td>
    <td>19.27x</td>
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
    <td style="white-space: nowrap">Kernel.++/2 (list)</td>
    <td style="white-space: nowrap; text-align: right">8328.70</td>
    <td style="white-space: nowrap; text-align: right">0.120 ms</td>
    <td style="white-space: nowrap; text-align: right">±82.13%</td>
    <td style="white-space: nowrap; text-align: right">0.0707 ms</td>
    <td style="white-space: nowrap; text-align: right">0.35 ms</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">135.44</td>
    <td style="white-space: nowrap; text-align: right">7.38 ms</td>
    <td style="white-space: nowrap; text-align: right">±2.98%</td>
    <td style="white-space: nowrap; text-align: right">7.35 ms</td>
    <td style="white-space: nowrap; text-align: right">7.76 ms</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">100.55</td>
    <td style="white-space: nowrap; text-align: right">9.95 ms</td>
    <td style="white-space: nowrap; text-align: right">±3.94%</td>
    <td style="white-space: nowrap; text-align: right">9.92 ms</td>
    <td style="white-space: nowrap; text-align: right">10.73 ms</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">34.66</td>
    <td style="white-space: nowrap; text-align: right">28.85 ms</td>
    <td style="white-space: nowrap; text-align: right">±1.33%</td>
    <td style="white-space: nowrap; text-align: right">28.83 ms</td>
    <td style="white-space: nowrap; text-align: right">29.55 ms</td>
  </tr>

</table>


Comparison

<table style="width: 1%">
  <tr>
    <th>Name</th>
    <th style="text-align: right">IPS</th>
    <th style="text-align: right">Slower</th>
  <tr>
    <td style="white-space: nowrap">Kernel.++/2 (list)</td>
    <td style="white-space: nowrap;text-align: right">8328.70</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">135.44</td>
    <td style="white-space: nowrap; text-align: right">61.49x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">100.55</td>
    <td style="white-space: nowrap; text-align: right">82.83x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">34.66</td>
    <td style="white-space: nowrap; text-align: right">240.28x</td>
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
    <td style="white-space: nowrap">Kernel.++/2 (list)</td>
    <td style="white-space: nowrap">0.0727 MB</td>
<td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (ErlangArray)</td>
    <td style="white-space: nowrap">9.64 MB</td>
    <td>132.63x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (MapArray)</td>
    <td style="white-space: nowrap">9.06 MB</td>
    <td>124.62x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (RRBVector)</td>
    <td style="white-space: nowrap">4.00 MB</td>
    <td>55.01x</td>
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
    <td style="white-space: nowrap">Kernel.++/2 (list)</td>
    <td style="white-space: nowrap; text-align: right">2876.28</td>
    <td style="white-space: nowrap; text-align: right">0.35 ms</td>
    <td style="white-space: nowrap; text-align: right">±66.09%</td>
    <td style="white-space: nowrap; text-align: right">0.169 ms</td>
    <td style="white-space: nowrap; text-align: right">0.72 ms</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">63.76</td>
    <td style="white-space: nowrap; text-align: right">15.68 ms</td>
    <td style="white-space: nowrap; text-align: right">±2.02%</td>
    <td style="white-space: nowrap; text-align: right">15.82 ms</td>
    <td style="white-space: nowrap; text-align: right">16.12 ms</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">38.96</td>
    <td style="white-space: nowrap; text-align: right">25.67 ms</td>
    <td style="white-space: nowrap; text-align: right">±7.03%</td>
    <td style="white-space: nowrap; text-align: right">25.25 ms</td>
    <td style="white-space: nowrap; text-align: right">29.21 ms</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">17.65</td>
    <td style="white-space: nowrap; text-align: right">56.65 ms</td>
    <td style="white-space: nowrap; text-align: right">±3.37%</td>
    <td style="white-space: nowrap; text-align: right">56.59 ms</td>
    <td style="white-space: nowrap; text-align: right">58.59 ms</td>
  </tr>

</table>


Comparison

<table style="width: 1%">
  <tr>
    <th>Name</th>
    <th style="text-align: right">IPS</th>
    <th style="text-align: right">Slower</th>
  <tr>
    <td style="white-space: nowrap">Kernel.++/2 (list)</td>
    <td style="white-space: nowrap;text-align: right">2876.28</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">63.76</td>
    <td style="white-space: nowrap; text-align: right">45.11x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">38.96</td>
    <td style="white-space: nowrap; text-align: right">73.83x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">17.65</td>
    <td style="white-space: nowrap; text-align: right">162.94x</td>
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
    <td style="white-space: nowrap">Kernel.++/2 (list)</td>
    <td style="white-space: nowrap">0.50 MB</td>
<td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (ErlangArray)</td>
    <td style="white-space: nowrap">19.30 MB</td>
    <td>38.61x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (MapArray)</td>
    <td style="white-space: nowrap">19.13 MB</td>
    <td>38.27x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (RRBVector)</td>
    <td style="white-space: nowrap">8.00 MB</td>
    <td>16.0x</td>
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
    <td style="white-space: nowrap">Kernel.++/2 (list)</td>
    <td style="white-space: nowrap; text-align: right">1026.40</td>
    <td style="white-space: nowrap; text-align: right">0.97 ms</td>
    <td style="white-space: nowrap; text-align: right">±81.77%</td>
    <td style="white-space: nowrap; text-align: right">0.57 ms</td>
    <td style="white-space: nowrap; text-align: right">2.28 ms</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">30.15</td>
    <td style="white-space: nowrap; text-align: right">33.17 ms</td>
    <td style="white-space: nowrap; text-align: right">±1.85%</td>
    <td style="white-space: nowrap; text-align: right">33.05 ms</td>
    <td style="white-space: nowrap; text-align: right">33.96 ms</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">13.23</td>
    <td style="white-space: nowrap; text-align: right">75.57 ms</td>
    <td style="white-space: nowrap; text-align: right">±12.89%</td>
    <td style="white-space: nowrap; text-align: right">81.09 ms</td>
    <td style="white-space: nowrap; text-align: right">81.31 ms</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">8.56</td>
    <td style="white-space: nowrap; text-align: right">116.78 ms</td>
    <td style="white-space: nowrap; text-align: right">±1.39%</td>
    <td style="white-space: nowrap; text-align: right">116.78 ms</td>
    <td style="white-space: nowrap; text-align: right">117.93 ms</td>
  </tr>

</table>


Comparison

<table style="width: 1%">
  <tr>
    <th>Name</th>
    <th style="text-align: right">IPS</th>
    <th style="text-align: right">Slower</th>
  <tr>
    <td style="white-space: nowrap">Kernel.++/2 (list)</td>
    <td style="white-space: nowrap;text-align: right">1026.40</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">30.15</td>
    <td style="white-space: nowrap; text-align: right">34.05x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">13.23</td>
    <td style="white-space: nowrap; text-align: right">77.57x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">8.56</td>
    <td style="white-space: nowrap; text-align: right">119.86x</td>
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
    <td style="white-space: nowrap">Kernel.++/2 (list)</td>
    <td style="white-space: nowrap">0.43 MB</td>
<td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (ErlangArray)</td>
    <td style="white-space: nowrap">41.45 MB</td>
    <td>97.03x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (MapArray)</td>
    <td style="white-space: nowrap">40.79 MB</td>
    <td>95.47x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (RRBVector)</td>
    <td style="white-space: nowrap">16.00 MB</td>
    <td>37.45x</td>
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
    <td style="white-space: nowrap">Kernel.++/2 (list)</td>
    <td style="white-space: nowrap; text-align: right">499.28</td>
    <td style="white-space: nowrap; text-align: right">2.00 ms</td>
    <td style="white-space: nowrap; text-align: right">±67.09%</td>
    <td style="white-space: nowrap; text-align: right">2.29 ms</td>
    <td style="white-space: nowrap; text-align: right">3.18 ms</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">13.85</td>
    <td style="white-space: nowrap; text-align: right">72.18 ms</td>
    <td style="white-space: nowrap; text-align: right">±0.54%</td>
    <td style="white-space: nowrap; text-align: right">72.18 ms</td>
    <td style="white-space: nowrap; text-align: right">72.46 ms</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">5.02</td>
    <td style="white-space: nowrap; text-align: right">199.27 ms</td>
    <td style="white-space: nowrap; text-align: right">±0.00%</td>
    <td style="white-space: nowrap; text-align: right">199.27 ms</td>
    <td style="white-space: nowrap; text-align: right">199.27 ms</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">4.16</td>
    <td style="white-space: nowrap; text-align: right">240.10 ms</td>
    <td style="white-space: nowrap; text-align: right">±0.00%</td>
    <td style="white-space: nowrap; text-align: right">240.10 ms</td>
    <td style="white-space: nowrap; text-align: right">240.10 ms</td>
  </tr>

</table>


Comparison

<table style="width: 1%">
  <tr>
    <th>Name</th>
    <th style="text-align: right">IPS</th>
    <th style="text-align: right">Slower</th>
  <tr>
    <td style="white-space: nowrap">Kernel.++/2 (list)</td>
    <td style="white-space: nowrap;text-align: right">499.28</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">13.85</td>
    <td style="white-space: nowrap; text-align: right">36.04x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">5.02</td>
    <td style="white-space: nowrap; text-align: right">99.49x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">4.16</td>
    <td style="white-space: nowrap; text-align: right">119.88x</td>
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
    <td style="white-space: nowrap">Kernel.++/2 (list)</td>
    <td style="white-space: nowrap">2 MB</td>
<td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (ErlangArray)</td>
    <td style="white-space: nowrap">89.23 MB</td>
    <td>44.61x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (MapArray)</td>
    <td style="white-space: nowrap">86.03 MB</td>
    <td>43.02x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (RRBVector)</td>
    <td style="white-space: nowrap">32.00 MB</td>
    <td>16.0x</td>
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
    <td style="white-space: nowrap">Kernel.++/2 (list)</td>
    <td style="white-space: nowrap; text-align: right">177.94</td>
    <td style="white-space: nowrap; text-align: right">5.62 ms</td>
    <td style="white-space: nowrap; text-align: right">±0.00%</td>
    <td style="white-space: nowrap; text-align: right">5.62 ms</td>
    <td style="white-space: nowrap; text-align: right">5.62 ms</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">7.01</td>
    <td style="white-space: nowrap; text-align: right">142.74 ms</td>
    <td style="white-space: nowrap; text-align: right">±0.00%</td>
    <td style="white-space: nowrap; text-align: right">142.74 ms</td>
    <td style="white-space: nowrap; text-align: right">142.74 ms</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">2.51</td>
    <td style="white-space: nowrap; text-align: right">398.79 ms</td>
    <td style="white-space: nowrap; text-align: right">±0.00%</td>
    <td style="white-space: nowrap; text-align: right">398.79 ms</td>
    <td style="white-space: nowrap; text-align: right">398.79 ms</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">2.11</td>
    <td style="white-space: nowrap; text-align: right">472.89 ms</td>
    <td style="white-space: nowrap; text-align: right">±0.00%</td>
    <td style="white-space: nowrap; text-align: right">472.89 ms</td>
    <td style="white-space: nowrap; text-align: right">472.89 ms</td>
  </tr>

</table>


Comparison

<table style="width: 1%">
  <tr>
    <th>Name</th>
    <th style="text-align: right">IPS</th>
    <th style="text-align: right">Slower</th>
  <tr>
    <td style="white-space: nowrap">Kernel.++/2 (list)</td>
    <td style="white-space: nowrap;text-align: right">177.94</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">7.01</td>
    <td style="white-space: nowrap; text-align: right">25.4x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">2.51</td>
    <td style="white-space: nowrap; text-align: right">70.96x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">2.11</td>
    <td style="white-space: nowrap; text-align: right">84.15x</td>
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
    <td style="white-space: nowrap">Kernel.++/2 (list)</td>
    <td style="white-space: nowrap">1.15 MB</td>
<td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (ErlangArray)</td>
    <td style="white-space: nowrap">178.46 MB</td>
    <td>154.97x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (MapArray)</td>
    <td style="white-space: nowrap">179.20 MB</td>
    <td>155.61x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (RRBVector)</td>
    <td style="white-space: nowrap">64.00 MB</td>
    <td>55.58x</td>
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
    <td style="white-space: nowrap">Kernel.++/2 (list)</td>
    <td style="white-space: nowrap; text-align: right">75.55</td>
    <td style="white-space: nowrap; text-align: right">13.24 ms</td>
    <td style="white-space: nowrap; text-align: right">±0.00%</td>
    <td style="white-space: nowrap; text-align: right">13.24 ms</td>
    <td style="white-space: nowrap; text-align: right">13.24 ms</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">3.54</td>
    <td style="white-space: nowrap; text-align: right">282.33 ms</td>
    <td style="white-space: nowrap; text-align: right">±0.00%</td>
    <td style="white-space: nowrap; text-align: right">282.33 ms</td>
    <td style="white-space: nowrap; text-align: right">282.33 ms</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">1.09</td>
    <td style="white-space: nowrap; text-align: right">916.07 ms</td>
    <td style="white-space: nowrap; text-align: right">±0.00%</td>
    <td style="white-space: nowrap; text-align: right">916.07 ms</td>
    <td style="white-space: nowrap; text-align: right">916.07 ms</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">1.04</td>
    <td style="white-space: nowrap; text-align: right">958.93 ms</td>
    <td style="white-space: nowrap; text-align: right">±0.00%</td>
    <td style="white-space: nowrap; text-align: right">958.93 ms</td>
    <td style="white-space: nowrap; text-align: right">958.93 ms</td>
  </tr>

</table>


Comparison

<table style="width: 1%">
  <tr>
    <th>Name</th>
    <th style="text-align: right">IPS</th>
    <th style="text-align: right">Slower</th>
  <tr>
    <td style="white-space: nowrap">Kernel.++/2 (list)</td>
    <td style="white-space: nowrap;text-align: right">75.55</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">3.54</td>
    <td style="white-space: nowrap; text-align: right">21.33x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">1.09</td>
    <td style="white-space: nowrap; text-align: right">69.21x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">1.04</td>
    <td style="white-space: nowrap; text-align: right">72.45x</td>
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
    <td style="white-space: nowrap">Kernel.++/2 (list)</td>
    <td style="white-space: nowrap">2.98 MB</td>
<td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (ErlangArray)</td>
    <td style="white-space: nowrap">361.37 MB</td>
    <td>121.39x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (MapArray)</td>
    <td style="white-space: nowrap">374.79 MB</td>
    <td>125.9x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (RRBVector)</td>
    <td style="white-space: nowrap">128.00 MB</td>
    <td>43.0x</td>
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
    <td style="white-space: nowrap">Kernel.++/2 (list)</td>
    <td style="white-space: nowrap; text-align: right">169.09</td>
    <td style="white-space: nowrap; text-align: right">0.00591 s</td>
    <td style="white-space: nowrap; text-align: right">±0.00%</td>
    <td style="white-space: nowrap; text-align: right">0.00591 s</td>
    <td style="white-space: nowrap; text-align: right">0.00591 s</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">1.55</td>
    <td style="white-space: nowrap; text-align: right">0.64 s</td>
    <td style="white-space: nowrap; text-align: right">±0.00%</td>
    <td style="white-space: nowrap; text-align: right">0.64 s</td>
    <td style="white-space: nowrap; text-align: right">0.64 s</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.51</td>
    <td style="white-space: nowrap; text-align: right">1.95 s</td>
    <td style="white-space: nowrap; text-align: right">±0.00%</td>
    <td style="white-space: nowrap; text-align: right">1.95 s</td>
    <td style="white-space: nowrap; text-align: right">1.95 s</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">0.48</td>
    <td style="white-space: nowrap; text-align: right">2.09 s</td>
    <td style="white-space: nowrap; text-align: right">±0.00%</td>
    <td style="white-space: nowrap; text-align: right">2.09 s</td>
    <td style="white-space: nowrap; text-align: right">2.09 s</td>
  </tr>

</table>


Comparison

<table style="width: 1%">
  <tr>
    <th>Name</th>
    <th style="text-align: right">IPS</th>
    <th style="text-align: right">Slower</th>
  <tr>
    <td style="white-space: nowrap">Kernel.++/2 (list)</td>
    <td style="white-space: nowrap;text-align: right">169.09</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">1.55</td>
    <td style="white-space: nowrap; text-align: right">108.88x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.51</td>
    <td style="white-space: nowrap; text-align: right">329.11x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">0.48</td>
    <td style="white-space: nowrap; text-align: right">352.87x</td>
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
    <td style="white-space: nowrap">Kernel.++/2 (list)</td>
    <td style="white-space: nowrap">0.79 MB</td>
<td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (ErlangArray)</td>
    <td style="white-space: nowrap">809.86 MB</td>
    <td>1021.91x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (RRBVector)</td>
    <td style="white-space: nowrap">256.00 MB</td>
    <td>323.03x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (MapArray)</td>
    <td style="white-space: nowrap">789.24 MB</td>
    <td>995.89x</td>
  </tr>

</table>


<hr/>

