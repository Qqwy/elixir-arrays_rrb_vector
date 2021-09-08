
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
    <td style="white-space: nowrap; text-align: right">4266.77 K</td>
    <td style="white-space: nowrap; text-align: right">0.23 μs</td>
    <td style="white-space: nowrap; text-align: right">±664.56%</td>
    <td style="white-space: nowrap; text-align: right">0.108 μs</td>
    <td style="white-space: nowrap; text-align: right">0.65 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">97.01 K</td>
    <td style="white-space: nowrap; text-align: right">10.31 μs</td>
    <td style="white-space: nowrap; text-align: right">±87.99%</td>
    <td style="white-space: nowrap; text-align: right">7.06 μs</td>
    <td style="white-space: nowrap; text-align: right">35.01 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">66.73 K</td>
    <td style="white-space: nowrap; text-align: right">14.99 μs</td>
    <td style="white-space: nowrap; text-align: right">±67.46%</td>
    <td style="white-space: nowrap; text-align: right">10.71 μs</td>
    <td style="white-space: nowrap; text-align: right">50.80 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">3.47 K</td>
    <td style="white-space: nowrap; text-align: right">287.86 μs</td>
    <td style="white-space: nowrap; text-align: right">±25.74%</td>
    <td style="white-space: nowrap; text-align: right">260.49 μs</td>
    <td style="white-space: nowrap; text-align: right">624.52 μs</td>
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
    <td style="white-space: nowrap;text-align: right">4266.77 K</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">97.01 K</td>
    <td style="white-space: nowrap; text-align: right">43.98x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">66.73 K</td>
    <td style="white-space: nowrap; text-align: right">63.94x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">3.47 K</td>
    <td style="white-space: nowrap; text-align: right">1228.24x</td>
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
    <td style="white-space: nowrap; text-align: right">1850.24 K</td>
    <td style="white-space: nowrap; text-align: right">0.54 μs</td>
    <td style="white-space: nowrap; text-align: right">±520.19%</td>
    <td style="white-space: nowrap; text-align: right">0.21 μs</td>
    <td style="white-space: nowrap; text-align: right">17.25 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">49.70 K</td>
    <td style="white-space: nowrap; text-align: right">20.12 μs</td>
    <td style="white-space: nowrap; text-align: right">±40.55%</td>
    <td style="white-space: nowrap; text-align: right">17.41 μs</td>
    <td style="white-space: nowrap; text-align: right">53.65 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">46.47 K</td>
    <td style="white-space: nowrap; text-align: right">21.52 μs</td>
    <td style="white-space: nowrap; text-align: right">±42.21%</td>
    <td style="white-space: nowrap; text-align: right">19.51 μs</td>
    <td style="white-space: nowrap; text-align: right">60.10 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">1.88 K</td>
    <td style="white-space: nowrap; text-align: right">530.67 μs</td>
    <td style="white-space: nowrap; text-align: right">±45.95%</td>
    <td style="white-space: nowrap; text-align: right">500.32 μs</td>
    <td style="white-space: nowrap; text-align: right">934.51 μs</td>
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
    <td style="white-space: nowrap;text-align: right">1850.24 K</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">49.70 K</td>
    <td style="white-space: nowrap; text-align: right">37.23x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">46.47 K</td>
    <td style="white-space: nowrap; text-align: right">39.81x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">1.88 K</td>
    <td style="white-space: nowrap; text-align: right">981.87x</td>
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
    <td style="white-space: nowrap; text-align: right">993.93 K</td>
    <td style="white-space: nowrap; text-align: right">1.01 μs</td>
    <td style="white-space: nowrap; text-align: right">±364.09%</td>
    <td style="white-space: nowrap; text-align: right">0.38 μs</td>
    <td style="white-space: nowrap; text-align: right">24.20 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">23.19 K</td>
    <td style="white-space: nowrap; text-align: right">43.12 μs</td>
    <td style="white-space: nowrap; text-align: right">±28.74%</td>
    <td style="white-space: nowrap; text-align: right">39.85 μs</td>
    <td style="white-space: nowrap; text-align: right">109.73 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">21.87 K</td>
    <td style="white-space: nowrap; text-align: right">45.73 μs</td>
    <td style="white-space: nowrap; text-align: right">±29.19%</td>
    <td style="white-space: nowrap; text-align: right">43.25 μs</td>
    <td style="white-space: nowrap; text-align: right">113.05 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.92 K</td>
    <td style="white-space: nowrap; text-align: right">1088.24 μs</td>
    <td style="white-space: nowrap; text-align: right">±14.55%</td>
    <td style="white-space: nowrap; text-align: right">1042.58 μs</td>
    <td style="white-space: nowrap; text-align: right">1599.48 μs</td>
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
    <td style="white-space: nowrap;text-align: right">993.93 K</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">23.19 K</td>
    <td style="white-space: nowrap; text-align: right">42.86x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">21.87 K</td>
    <td style="white-space: nowrap; text-align: right">45.45x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.92 K</td>
    <td style="white-space: nowrap; text-align: right">1081.64x</td>
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
    <td style="white-space: nowrap; text-align: right">654.80 K</td>
    <td style="white-space: nowrap; text-align: right">1.53 μs</td>
    <td style="white-space: nowrap; text-align: right">±186.00%</td>
    <td style="white-space: nowrap; text-align: right">0.71 μs</td>
    <td style="white-space: nowrap; text-align: right">10.92 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">11.81 K</td>
    <td style="white-space: nowrap; text-align: right">84.68 μs</td>
    <td style="white-space: nowrap; text-align: right">±25.93%</td>
    <td style="white-space: nowrap; text-align: right">78.85 μs</td>
    <td style="white-space: nowrap; text-align: right">189.24 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">9.20 K</td>
    <td style="white-space: nowrap; text-align: right">108.71 μs</td>
    <td style="white-space: nowrap; text-align: right">±24.17%</td>
    <td style="white-space: nowrap; text-align: right">101.58 μs</td>
    <td style="white-space: nowrap; text-align: right">231.16 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.44 K</td>
    <td style="white-space: nowrap; text-align: right">2253.85 μs</td>
    <td style="white-space: nowrap; text-align: right">±9.19%</td>
    <td style="white-space: nowrap; text-align: right">2182.95 μs</td>
    <td style="white-space: nowrap; text-align: right">2926.43 μs</td>
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
    <td style="white-space: nowrap;text-align: right">654.80 K</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">11.81 K</td>
    <td style="white-space: nowrap; text-align: right">55.45x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">9.20 K</td>
    <td style="white-space: nowrap; text-align: right">71.18x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.44 K</td>
    <td style="white-space: nowrap; text-align: right">1475.83x</td>
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
    <td style="white-space: nowrap; text-align: right">274.74 K</td>
    <td style="white-space: nowrap; text-align: right">3.64 μs</td>
    <td style="white-space: nowrap; text-align: right">±116.94%</td>
    <td style="white-space: nowrap; text-align: right">1.51 μs</td>
    <td style="white-space: nowrap; text-align: right">20.94 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">5.81 K</td>
    <td style="white-space: nowrap; text-align: right">172.15 μs</td>
    <td style="white-space: nowrap; text-align: right">±25.33%</td>
    <td style="white-space: nowrap; text-align: right">161.14 μs</td>
    <td style="white-space: nowrap; text-align: right">401.89 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">3.87 K</td>
    <td style="white-space: nowrap; text-align: right">258.62 μs</td>
    <td style="white-space: nowrap; text-align: right">±20.43%</td>
    <td style="white-space: nowrap; text-align: right">246.53 μs</td>
    <td style="white-space: nowrap; text-align: right">496.79 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.22 K</td>
    <td style="white-space: nowrap; text-align: right">4477.89 μs</td>
    <td style="white-space: nowrap; text-align: right">±5.07%</td>
    <td style="white-space: nowrap; text-align: right">4502.06 μs</td>
    <td style="white-space: nowrap; text-align: right">4928.90 μs</td>
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
    <td style="white-space: nowrap;text-align: right">274.74 K</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">5.81 K</td>
    <td style="white-space: nowrap; text-align: right">47.3x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">3.87 K</td>
    <td style="white-space: nowrap; text-align: right">71.05x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.22 K</td>
    <td style="white-space: nowrap; text-align: right">1230.25x</td>
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
    <td style="white-space: nowrap; text-align: right">132.71 K</td>
    <td style="white-space: nowrap; text-align: right">7.53 μs</td>
    <td style="white-space: nowrap; text-align: right">±92.30%</td>
    <td style="white-space: nowrap; text-align: right">3.14 μs</td>
    <td style="white-space: nowrap; text-align: right">32.55 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">2.54 K</td>
    <td style="white-space: nowrap; text-align: right">393.95 μs</td>
    <td style="white-space: nowrap; text-align: right">±18.07%</td>
    <td style="white-space: nowrap; text-align: right">376.86 μs</td>
    <td style="white-space: nowrap; text-align: right">802.55 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">1.95 K</td>
    <td style="white-space: nowrap; text-align: right">513.08 μs</td>
    <td style="white-space: nowrap; text-align: right">±15.64%</td>
    <td style="white-space: nowrap; text-align: right">494.91 μs</td>
    <td style="white-space: nowrap; text-align: right">849.16 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.109 K</td>
    <td style="white-space: nowrap; text-align: right">9154.33 μs</td>
    <td style="white-space: nowrap; text-align: right">±8.56%</td>
    <td style="white-space: nowrap; text-align: right">8978.77 μs</td>
    <td style="white-space: nowrap; text-align: right">12596.27 μs</td>
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
    <td style="white-space: nowrap;text-align: right">132.71 K</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">2.54 K</td>
    <td style="white-space: nowrap; text-align: right">52.28x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">1.95 K</td>
    <td style="white-space: nowrap; text-align: right">68.09x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.109 K</td>
    <td style="white-space: nowrap; text-align: right">1214.91x</td>
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
    <td style="white-space: nowrap; text-align: right">55.03 K</td>
    <td style="white-space: nowrap; text-align: right">0.0182 ms</td>
    <td style="white-space: nowrap; text-align: right">±67.32%</td>
    <td style="white-space: nowrap; text-align: right">0.0228 ms</td>
    <td style="white-space: nowrap; text-align: right">0.0551 ms</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">1.28 K</td>
    <td style="white-space: nowrap; text-align: right">0.78 ms</td>
    <td style="white-space: nowrap; text-align: right">±14.10%</td>
    <td style="white-space: nowrap; text-align: right">0.75 ms</td>
    <td style="white-space: nowrap; text-align: right">1.32 ms</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">0.92 K</td>
    <td style="white-space: nowrap; text-align: right">1.08 ms</td>
    <td style="white-space: nowrap; text-align: right">±14.61%</td>
    <td style="white-space: nowrap; text-align: right">1.06 ms</td>
    <td style="white-space: nowrap; text-align: right">1.58 ms</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.0547 K</td>
    <td style="white-space: nowrap; text-align: right">18.27 ms</td>
    <td style="white-space: nowrap; text-align: right">±1.83%</td>
    <td style="white-space: nowrap; text-align: right">18.26 ms</td>
    <td style="white-space: nowrap; text-align: right">18.71 ms</td>
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
    <td style="white-space: nowrap;text-align: right">55.03 K</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">1.28 K</td>
    <td style="white-space: nowrap; text-align: right">43.05x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">0.92 K</td>
    <td style="white-space: nowrap; text-align: right">59.64x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.0547 K</td>
    <td style="white-space: nowrap; text-align: right">1005.43x</td>
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
    <td style="white-space: nowrap; text-align: right">30568.67</td>
    <td style="white-space: nowrap; text-align: right">0.0327 ms</td>
    <td style="white-space: nowrap; text-align: right">±79.21%</td>
    <td style="white-space: nowrap; text-align: right">0.0134 ms</td>
    <td style="white-space: nowrap; text-align: right">0.116 ms</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">645.23</td>
    <td style="white-space: nowrap; text-align: right">1.55 ms</td>
    <td style="white-space: nowrap; text-align: right">±11.01%</td>
    <td style="white-space: nowrap; text-align: right">1.48 ms</td>
    <td style="white-space: nowrap; text-align: right">1.97 ms</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">453.89</td>
    <td style="white-space: nowrap; text-align: right">2.20 ms</td>
    <td style="white-space: nowrap; text-align: right">±9.25%</td>
    <td style="white-space: nowrap; text-align: right">2.14 ms</td>
    <td style="white-space: nowrap; text-align: right">2.91 ms</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">28.00</td>
    <td style="white-space: nowrap; text-align: right">35.72 ms</td>
    <td style="white-space: nowrap; text-align: right">±3.09%</td>
    <td style="white-space: nowrap; text-align: right">35.21 ms</td>
    <td style="white-space: nowrap; text-align: right">37.64 ms</td>
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
    <td style="white-space: nowrap;text-align: right">30568.67</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">645.23</td>
    <td style="white-space: nowrap; text-align: right">47.38x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">453.89</td>
    <td style="white-space: nowrap; text-align: right">67.35x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">28.00</td>
    <td style="white-space: nowrap; text-align: right">1091.82x</td>
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
    <td style="white-space: nowrap; text-align: right">19115.10</td>
    <td style="white-space: nowrap; text-align: right">0.0523 ms</td>
    <td style="white-space: nowrap; text-align: right">±79.87%</td>
    <td style="white-space: nowrap; text-align: right">0.0288 ms</td>
    <td style="white-space: nowrap; text-align: right">0.184 ms</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">272.86</td>
    <td style="white-space: nowrap; text-align: right">3.66 ms</td>
    <td style="white-space: nowrap; text-align: right">±7.51%</td>
    <td style="white-space: nowrap; text-align: right">3.66 ms</td>
    <td style="white-space: nowrap; text-align: right">4.35 ms</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">206.33</td>
    <td style="white-space: nowrap; text-align: right">4.85 ms</td>
    <td style="white-space: nowrap; text-align: right">±6.00%</td>
    <td style="white-space: nowrap; text-align: right">4.78 ms</td>
    <td style="white-space: nowrap; text-align: right">5.42 ms</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">12.89</td>
    <td style="white-space: nowrap; text-align: right">77.58 ms</td>
    <td style="white-space: nowrap; text-align: right">±2.45%</td>
    <td style="white-space: nowrap; text-align: right">76.82 ms</td>
    <td style="white-space: nowrap; text-align: right">79.74 ms</td>
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
    <td style="white-space: nowrap;text-align: right">19115.10</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">272.86</td>
    <td style="white-space: nowrap; text-align: right">70.05x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">206.33</td>
    <td style="white-space: nowrap; text-align: right">92.64x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">12.89</td>
    <td style="white-space: nowrap; text-align: right">1482.87x</td>
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
    <td style="white-space: nowrap; text-align: right">9093.94</td>
    <td style="white-space: nowrap; text-align: right">0.110 ms</td>
    <td style="white-space: nowrap; text-align: right">±90.12%</td>
    <td style="white-space: nowrap; text-align: right">0.0666 ms</td>
    <td style="white-space: nowrap; text-align: right">0.35 ms</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">140.79</td>
    <td style="white-space: nowrap; text-align: right">7.10 ms</td>
    <td style="white-space: nowrap; text-align: right">±4.06%</td>
    <td style="white-space: nowrap; text-align: right">7.10 ms</td>
    <td style="white-space: nowrap; text-align: right">7.58 ms</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">97.01</td>
    <td style="white-space: nowrap; text-align: right">10.31 ms</td>
    <td style="white-space: nowrap; text-align: right">±5.33%</td>
    <td style="white-space: nowrap; text-align: right">10.21 ms</td>
    <td style="white-space: nowrap; text-align: right">11.88 ms</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">6.40</td>
    <td style="white-space: nowrap; text-align: right">156.19 ms</td>
    <td style="white-space: nowrap; text-align: right">±4.29%</td>
    <td style="white-space: nowrap; text-align: right">156.19 ms</td>
    <td style="white-space: nowrap; text-align: right">160.92 ms</td>
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
    <td style="white-space: nowrap;text-align: right">9093.94</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">140.79</td>
    <td style="white-space: nowrap; text-align: right">64.59x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">97.01</td>
    <td style="white-space: nowrap; text-align: right">93.74x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">6.40</td>
    <td style="white-space: nowrap; text-align: right">1420.35x</td>
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
    <td style="white-space: nowrap; text-align: right">3628.91</td>
    <td style="white-space: nowrap; text-align: right">0.28 ms</td>
    <td style="white-space: nowrap; text-align: right">±81.36%</td>
    <td style="white-space: nowrap; text-align: right">0.140 ms</td>
    <td style="white-space: nowrap; text-align: right">0.67 ms</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">64.78</td>
    <td style="white-space: nowrap; text-align: right">15.44 ms</td>
    <td style="white-space: nowrap; text-align: right">±0.71%</td>
    <td style="white-space: nowrap; text-align: right">15.42 ms</td>
    <td style="white-space: nowrap; text-align: right">15.59 ms</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">38.28</td>
    <td style="white-space: nowrap; text-align: right">26.13 ms</td>
    <td style="white-space: nowrap; text-align: right">±7.44%</td>
    <td style="white-space: nowrap; text-align: right">25.44 ms</td>
    <td style="white-space: nowrap; text-align: right">30.04 ms</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">3.21</td>
    <td style="white-space: nowrap; text-align: right">311.81 ms</td>
    <td style="white-space: nowrap; text-align: right">±0.00%</td>
    <td style="white-space: nowrap; text-align: right">311.81 ms</td>
    <td style="white-space: nowrap; text-align: right">311.81 ms</td>
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
    <td style="white-space: nowrap;text-align: right">3628.91</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">64.78</td>
    <td style="white-space: nowrap; text-align: right">56.01x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">38.28</td>
    <td style="white-space: nowrap; text-align: right">94.81x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">3.21</td>
    <td style="white-space: nowrap; text-align: right">1131.51x</td>
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
    <td style="white-space: nowrap; text-align: right">774.24</td>
    <td style="white-space: nowrap; text-align: right">1.29 ms</td>
    <td style="white-space: nowrap; text-align: right">±53.73%</td>
    <td style="white-space: nowrap; text-align: right">1.22 ms</td>
    <td style="white-space: nowrap; text-align: right">2.16 ms</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">30.58</td>
    <td style="white-space: nowrap; text-align: right">32.70 ms</td>
    <td style="white-space: nowrap; text-align: right">±1.45%</td>
    <td style="white-space: nowrap; text-align: right">32.62 ms</td>
    <td style="white-space: nowrap; text-align: right">33.33 ms</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">12.25</td>
    <td style="white-space: nowrap; text-align: right">81.62 ms</td>
    <td style="white-space: nowrap; text-align: right">±1.26%</td>
    <td style="white-space: nowrap; text-align: right">82.14 ms</td>
    <td style="white-space: nowrap; text-align: right">82.28 ms</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">1.60</td>
    <td style="white-space: nowrap; text-align: right">626.62 ms</td>
    <td style="white-space: nowrap; text-align: right">±0.00%</td>
    <td style="white-space: nowrap; text-align: right">626.62 ms</td>
    <td style="white-space: nowrap; text-align: right">626.62 ms</td>
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
    <td style="white-space: nowrap;text-align: right">774.24</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">30.58</td>
    <td style="white-space: nowrap; text-align: right">25.32x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">12.25</td>
    <td style="white-space: nowrap; text-align: right">63.19x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">1.60</td>
    <td style="white-space: nowrap; text-align: right">485.15x</td>
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
    <td style="white-space: nowrap; text-align: right">655.93</td>
    <td style="white-space: nowrap; text-align: right">1.52 ms</td>
    <td style="white-space: nowrap; text-align: right">±107.08%</td>
    <td style="white-space: nowrap; text-align: right">0.59 ms</td>
    <td style="white-space: nowrap; text-align: right">3.41 ms</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">13.95</td>
    <td style="white-space: nowrap; text-align: right">71.70 ms</td>
    <td style="white-space: nowrap; text-align: right">±2.44%</td>
    <td style="white-space: nowrap; text-align: right">71.70 ms</td>
    <td style="white-space: nowrap; text-align: right">72.93 ms</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">6.22</td>
    <td style="white-space: nowrap; text-align: right">160.87 ms</td>
    <td style="white-space: nowrap; text-align: right">±0.00%</td>
    <td style="white-space: nowrap; text-align: right">160.87 ms</td>
    <td style="white-space: nowrap; text-align: right">160.87 ms</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.80</td>
    <td style="white-space: nowrap; text-align: right">1253.35 ms</td>
    <td style="white-space: nowrap; text-align: right">±0.00%</td>
    <td style="white-space: nowrap; text-align: right">1253.35 ms</td>
    <td style="white-space: nowrap; text-align: right">1253.35 ms</td>
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
    <td style="white-space: nowrap;text-align: right">655.93</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">13.95</td>
    <td style="white-space: nowrap; text-align: right">47.03x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">6.22</td>
    <td style="white-space: nowrap; text-align: right">105.52x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.80</td>
    <td style="white-space: nowrap; text-align: right">822.12x</td>
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
    <td style="white-space: nowrap; text-align: right">167.61</td>
    <td style="white-space: nowrap; text-align: right">5.97 ms</td>
    <td style="white-space: nowrap; text-align: right">±0.00%</td>
    <td style="white-space: nowrap; text-align: right">5.97 ms</td>
    <td style="white-space: nowrap; text-align: right">5.97 ms</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">7.36</td>
    <td style="white-space: nowrap; text-align: right">135.94 ms</td>
    <td style="white-space: nowrap; text-align: right">±0.00%</td>
    <td style="white-space: nowrap; text-align: right">135.94 ms</td>
    <td style="white-space: nowrap; text-align: right">135.94 ms</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">2.49</td>
    <td style="white-space: nowrap; text-align: right">401.30 ms</td>
    <td style="white-space: nowrap; text-align: right">±0.00%</td>
    <td style="white-space: nowrap; text-align: right">401.30 ms</td>
    <td style="white-space: nowrap; text-align: right">401.30 ms</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.39</td>
    <td style="white-space: nowrap; text-align: right">2564.59 ms</td>
    <td style="white-space: nowrap; text-align: right">±0.00%</td>
    <td style="white-space: nowrap; text-align: right">2564.59 ms</td>
    <td style="white-space: nowrap; text-align: right">2564.59 ms</td>
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
    <td style="white-space: nowrap;text-align: right">167.61</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">7.36</td>
    <td style="white-space: nowrap; text-align: right">22.78x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">2.49</td>
    <td style="white-space: nowrap; text-align: right">67.26x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.39</td>
    <td style="white-space: nowrap; text-align: right">429.86x</td>
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
    <td style="white-space: nowrap; text-align: right">441.96</td>
    <td style="white-space: nowrap; text-align: right">2.26 ms</td>
    <td style="white-space: nowrap; text-align: right">±0.00%</td>
    <td style="white-space: nowrap; text-align: right">2.26 ms</td>
    <td style="white-space: nowrap; text-align: right">2.26 ms</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">3.57</td>
    <td style="white-space: nowrap; text-align: right">279.99 ms</td>
    <td style="white-space: nowrap; text-align: right">±0.00%</td>
    <td style="white-space: nowrap; text-align: right">279.99 ms</td>
    <td style="white-space: nowrap; text-align: right">279.99 ms</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">1.12</td>
    <td style="white-space: nowrap; text-align: right">894.15 ms</td>
    <td style="white-space: nowrap; text-align: right">±0.00%</td>
    <td style="white-space: nowrap; text-align: right">894.15 ms</td>
    <td style="white-space: nowrap; text-align: right">894.15 ms</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.194</td>
    <td style="white-space: nowrap; text-align: right">5150.17 ms</td>
    <td style="white-space: nowrap; text-align: right">±0.00%</td>
    <td style="white-space: nowrap; text-align: right">5150.17 ms</td>
    <td style="white-space: nowrap; text-align: right">5150.17 ms</td>
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
    <td style="white-space: nowrap;text-align: right">441.96</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">3.57</td>
    <td style="white-space: nowrap; text-align: right">123.75x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">1.12</td>
    <td style="white-space: nowrap; text-align: right">395.18x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.194</td>
    <td style="white-space: nowrap; text-align: right">2276.18x</td>
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
    <td style="white-space: nowrap; text-align: right">209.09</td>
    <td style="white-space: nowrap; text-align: right">0.00478 s</td>
    <td style="white-space: nowrap; text-align: right">±0.00%</td>
    <td style="white-space: nowrap; text-align: right">0.00478 s</td>
    <td style="white-space: nowrap; text-align: right">0.00478 s</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">1.43</td>
    <td style="white-space: nowrap; text-align: right">0.70 s</td>
    <td style="white-space: nowrap; text-align: right">±0.00%</td>
    <td style="white-space: nowrap; text-align: right">0.70 s</td>
    <td style="white-space: nowrap; text-align: right">0.70 s</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">0.44</td>
    <td style="white-space: nowrap; text-align: right">2.26 s</td>
    <td style="white-space: nowrap; text-align: right">±0.00%</td>
    <td style="white-space: nowrap; text-align: right">2.26 s</td>
    <td style="white-space: nowrap; text-align: right">2.26 s</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.0956</td>
    <td style="white-space: nowrap; text-align: right">10.46 s</td>
    <td style="white-space: nowrap; text-align: right">±0.00%</td>
    <td style="white-space: nowrap; text-align: right">10.46 s</td>
    <td style="white-space: nowrap; text-align: right">10.46 s</td>
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
    <td style="white-space: nowrap;text-align: right">209.09</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">1.43</td>
    <td style="white-space: nowrap; text-align: right">146.21x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">0.44</td>
    <td style="white-space: nowrap; text-align: right">472.8x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.0956</td>
    <td style="white-space: nowrap; text-align: right">2187.4x</td>
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
    <td style="white-space: nowrap">Arrays.concat/2 (MapArray)</td>
    <td style="white-space: nowrap">789.24 MB</td>
    <td>995.89x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (RRBVector)</td>
    <td style="white-space: nowrap">256.00 MB</td>
    <td>323.03x</td>
  </tr>

</table>


<hr/>

