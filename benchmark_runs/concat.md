
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
    <td style="white-space: nowrap; text-align: right">3336.80 K</td>
    <td style="white-space: nowrap; text-align: right">0.30 μs</td>
    <td style="white-space: nowrap; text-align: right">±723.61%</td>
    <td style="white-space: nowrap; text-align: right">0.110 μs</td>
    <td style="white-space: nowrap; text-align: right">2.20 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">95.15 K</td>
    <td style="white-space: nowrap; text-align: right">10.51 μs</td>
    <td style="white-space: nowrap; text-align: right">±88.13%</td>
    <td style="white-space: nowrap; text-align: right">7.15 μs</td>
    <td style="white-space: nowrap; text-align: right">38.56 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">64.84 K</td>
    <td style="white-space: nowrap; text-align: right">15.42 μs</td>
    <td style="white-space: nowrap; text-align: right">±67.64%</td>
    <td style="white-space: nowrap; text-align: right">11.08 μs</td>
    <td style="white-space: nowrap; text-align: right">52.90 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">20.94 K</td>
    <td style="white-space: nowrap; text-align: right">47.75 μs</td>
    <td style="white-space: nowrap; text-align: right">±78.18%</td>
    <td style="white-space: nowrap; text-align: right">37.15 μs</td>
    <td style="white-space: nowrap; text-align: right">131.65 μs</td>
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
    <td style="white-space: nowrap;text-align: right">3336.80 K</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">95.15 K</td>
    <td style="white-space: nowrap; text-align: right">35.07x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">64.84 K</td>
    <td style="white-space: nowrap; text-align: right">51.46x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">20.94 K</td>
    <td style="white-space: nowrap; text-align: right">159.33x</td>
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
    <td style="white-space: nowrap; text-align: right">2289.91 K</td>
    <td style="white-space: nowrap; text-align: right">0.44 μs</td>
    <td style="white-space: nowrap; text-align: right">±530.16%</td>
    <td style="white-space: nowrap; text-align: right">0.191 μs</td>
    <td style="white-space: nowrap; text-align: right">10.91 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">49.49 K</td>
    <td style="white-space: nowrap; text-align: right">20.21 μs</td>
    <td style="white-space: nowrap; text-align: right">±43.72%</td>
    <td style="white-space: nowrap; text-align: right">17.23 μs</td>
    <td style="white-space: nowrap; text-align: right">58.32 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">45.10 K</td>
    <td style="white-space: nowrap; text-align: right">22.17 μs</td>
    <td style="white-space: nowrap; text-align: right">±38.36%</td>
    <td style="white-space: nowrap; text-align: right">20.07 μs</td>
    <td style="white-space: nowrap; text-align: right">58.22 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">9.84 K</td>
    <td style="white-space: nowrap; text-align: right">101.60 μs</td>
    <td style="white-space: nowrap; text-align: right">±84.54%</td>
    <td style="white-space: nowrap; text-align: right">86.21 μs</td>
    <td style="white-space: nowrap; text-align: right">254.50 μs</td>
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
    <td style="white-space: nowrap;text-align: right">2289.91 K</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">49.49 K</td>
    <td style="white-space: nowrap; text-align: right">46.27x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">45.10 K</td>
    <td style="white-space: nowrap; text-align: right">50.77x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">9.84 K</td>
    <td style="white-space: nowrap; text-align: right">232.65x</td>
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
    <td style="white-space: nowrap; text-align: right">865.84 K</td>
    <td style="white-space: nowrap; text-align: right">1.15 μs</td>
    <td style="white-space: nowrap; text-align: right">±360.66%</td>
    <td style="white-space: nowrap; text-align: right">0.41 μs</td>
    <td style="white-space: nowrap; text-align: right">27.01 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">22.84 K</td>
    <td style="white-space: nowrap; text-align: right">43.78 μs</td>
    <td style="white-space: nowrap; text-align: right">±25.84%</td>
    <td style="white-space: nowrap; text-align: right">41.08 μs</td>
    <td style="white-space: nowrap; text-align: right">98.16 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">20.65 K</td>
    <td style="white-space: nowrap; text-align: right">48.42 μs</td>
    <td style="white-space: nowrap; text-align: right">±30.11%</td>
    <td style="white-space: nowrap; text-align: right">46.04 μs</td>
    <td style="white-space: nowrap; text-align: right">120.36 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">4.73 K</td>
    <td style="white-space: nowrap; text-align: right">211.61 μs</td>
    <td style="white-space: nowrap; text-align: right">±28.53%</td>
    <td style="white-space: nowrap; text-align: right">207.80 μs</td>
    <td style="white-space: nowrap; text-align: right">431.48 μs</td>
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
    <td style="white-space: nowrap;text-align: right">865.84 K</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">22.84 K</td>
    <td style="white-space: nowrap; text-align: right">37.91x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">20.65 K</td>
    <td style="white-space: nowrap; text-align: right">41.93x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">4.73 K</td>
    <td style="white-space: nowrap; text-align: right">183.22x</td>
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
    <td style="white-space: nowrap; text-align: right">620.06 K</td>
    <td style="white-space: nowrap; text-align: right">1.61 μs</td>
    <td style="white-space: nowrap; text-align: right">±176.12%</td>
    <td style="white-space: nowrap; text-align: right">0.75 μs</td>
    <td style="white-space: nowrap; text-align: right">10.36 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">11.50 K</td>
    <td style="white-space: nowrap; text-align: right">86.97 μs</td>
    <td style="white-space: nowrap; text-align: right">±25.12%</td>
    <td style="white-space: nowrap; text-align: right">79.87 μs</td>
    <td style="white-space: nowrap; text-align: right">177.64 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">8.76 K</td>
    <td style="white-space: nowrap; text-align: right">114.15 μs</td>
    <td style="white-space: nowrap; text-align: right">±24.56%</td>
    <td style="white-space: nowrap; text-align: right">107.98 μs</td>
    <td style="white-space: nowrap; text-align: right">267.43 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">2.16 K</td>
    <td style="white-space: nowrap; text-align: right">463.61 μs</td>
    <td style="white-space: nowrap; text-align: right">±21.00%</td>
    <td style="white-space: nowrap; text-align: right">445.63 μs</td>
    <td style="white-space: nowrap; text-align: right">901.05 μs</td>
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
    <td style="white-space: nowrap;text-align: right">620.06 K</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">11.50 K</td>
    <td style="white-space: nowrap; text-align: right">53.93x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">8.76 K</td>
    <td style="white-space: nowrap; text-align: right">70.78x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">2.16 K</td>
    <td style="white-space: nowrap; text-align: right">287.47x</td>
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
    <td style="white-space: nowrap; text-align: right">270.48 K</td>
    <td style="white-space: nowrap; text-align: right">3.70 μs</td>
    <td style="white-space: nowrap; text-align: right">±108.92%</td>
    <td style="white-space: nowrap; text-align: right">1.57 μs</td>
    <td style="white-space: nowrap; text-align: right">21.23 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">5.77 K</td>
    <td style="white-space: nowrap; text-align: right">173.31 μs</td>
    <td style="white-space: nowrap; text-align: right">±21.78%</td>
    <td style="white-space: nowrap; text-align: right">163.35 μs</td>
    <td style="white-space: nowrap; text-align: right">387.50 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">3.81 K</td>
    <td style="white-space: nowrap; text-align: right">262.80 μs</td>
    <td style="white-space: nowrap; text-align: right">±18.41%</td>
    <td style="white-space: nowrap; text-align: right">252.37 μs</td>
    <td style="white-space: nowrap; text-align: right">459.28 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">1.02 K</td>
    <td style="white-space: nowrap; text-align: right">977.66 μs</td>
    <td style="white-space: nowrap; text-align: right">±38.67%</td>
    <td style="white-space: nowrap; text-align: right">925.15 μs</td>
    <td style="white-space: nowrap; text-align: right">1390.57 μs</td>
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
    <td style="white-space: nowrap;text-align: right">270.48 K</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">5.77 K</td>
    <td style="white-space: nowrap; text-align: right">46.88x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">3.81 K</td>
    <td style="white-space: nowrap; text-align: right">71.08x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">1.02 K</td>
    <td style="white-space: nowrap; text-align: right">264.44x</td>
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
    <td style="white-space: nowrap; text-align: right">126.95 K</td>
    <td style="white-space: nowrap; text-align: right">7.88 μs</td>
    <td style="white-space: nowrap; text-align: right">±87.96%</td>
    <td style="white-space: nowrap; text-align: right">3.32 μs</td>
    <td style="white-space: nowrap; text-align: right">32.75 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">2.49 K</td>
    <td style="white-space: nowrap; text-align: right">400.86 μs</td>
    <td style="white-space: nowrap; text-align: right">±16.28%</td>
    <td style="white-space: nowrap; text-align: right">381.11 μs</td>
    <td style="white-space: nowrap; text-align: right">689.04 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">1.91 K</td>
    <td style="white-space: nowrap; text-align: right">523.49 μs</td>
    <td style="white-space: nowrap; text-align: right">±17.07%</td>
    <td style="white-space: nowrap; text-align: right">499.18 μs</td>
    <td style="white-space: nowrap; text-align: right">882.79 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.52 K</td>
    <td style="white-space: nowrap; text-align: right">1920.15 μs</td>
    <td style="white-space: nowrap; text-align: right">±25.57%</td>
    <td style="white-space: nowrap; text-align: right">1831.81 μs</td>
    <td style="white-space: nowrap; text-align: right">6366.91 μs</td>
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
    <td style="white-space: nowrap;text-align: right">126.95 K</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">2.49 K</td>
    <td style="white-space: nowrap; text-align: right">50.89x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">1.91 K</td>
    <td style="white-space: nowrap; text-align: right">66.45x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.52 K</td>
    <td style="white-space: nowrap; text-align: right">243.76x</td>
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
    <td style="white-space: nowrap; text-align: right">57.72 K</td>
    <td style="white-space: nowrap; text-align: right">0.0173 ms</td>
    <td style="white-space: nowrap; text-align: right">±79.89%</td>
    <td style="white-space: nowrap; text-align: right">0.00703 ms</td>
    <td style="white-space: nowrap; text-align: right">0.0696 ms</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">1.23 K</td>
    <td style="white-space: nowrap; text-align: right">0.82 ms</td>
    <td style="white-space: nowrap; text-align: right">±13.94%</td>
    <td style="white-space: nowrap; text-align: right">0.78 ms</td>
    <td style="white-space: nowrap; text-align: right">1.28 ms</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">0.93 K</td>
    <td style="white-space: nowrap; text-align: right">1.08 ms</td>
    <td style="white-space: nowrap; text-align: right">±13.85%</td>
    <td style="white-space: nowrap; text-align: right">1.02 ms</td>
    <td style="white-space: nowrap; text-align: right">1.62 ms</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.27 K</td>
    <td style="white-space: nowrap; text-align: right">3.68 ms</td>
    <td style="white-space: nowrap; text-align: right">±5.79%</td>
    <td style="white-space: nowrap; text-align: right">3.65 ms</td>
    <td style="white-space: nowrap; text-align: right">4.29 ms</td>
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
    <td style="white-space: nowrap;text-align: right">57.72 K</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">1.23 K</td>
    <td style="white-space: nowrap; text-align: right">47.04x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">0.93 K</td>
    <td style="white-space: nowrap; text-align: right">62.21x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.27 K</td>
    <td style="white-space: nowrap; text-align: right">212.31x</td>
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
    <td style="white-space: nowrap; text-align: right">30641.46</td>
    <td style="white-space: nowrap; text-align: right">0.0326 ms</td>
    <td style="white-space: nowrap; text-align: right">±73.80%</td>
    <td style="white-space: nowrap; text-align: right">0.0134 ms</td>
    <td style="white-space: nowrap; text-align: right">0.0919 ms</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">666.51</td>
    <td style="white-space: nowrap; text-align: right">1.50 ms</td>
    <td style="white-space: nowrap; text-align: right">±10.30%</td>
    <td style="white-space: nowrap; text-align: right">1.45 ms</td>
    <td style="white-space: nowrap; text-align: right">2.07 ms</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">451.46</td>
    <td style="white-space: nowrap; text-align: right">2.22 ms</td>
    <td style="white-space: nowrap; text-align: right">±9.58%</td>
    <td style="white-space: nowrap; text-align: right">2.19 ms</td>
    <td style="white-space: nowrap; text-align: right">3.16 ms</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">127.90</td>
    <td style="white-space: nowrap; text-align: right">7.82 ms</td>
    <td style="white-space: nowrap; text-align: right">±4.21%</td>
    <td style="white-space: nowrap; text-align: right">7.74 ms</td>
    <td style="white-space: nowrap; text-align: right">8.62 ms</td>
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
    <td style="white-space: nowrap;text-align: right">30641.46</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">666.51</td>
    <td style="white-space: nowrap; text-align: right">45.97x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">451.46</td>
    <td style="white-space: nowrap; text-align: right">67.87x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">127.90</td>
    <td style="white-space: nowrap; text-align: right">239.57x</td>
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
    <td style="white-space: nowrap; text-align: right">14300.20</td>
    <td style="white-space: nowrap; text-align: right">0.0699 ms</td>
    <td style="white-space: nowrap; text-align: right">±101.01%</td>
    <td style="white-space: nowrap; text-align: right">0.0308 ms</td>
    <td style="white-space: nowrap; text-align: right">0.37 ms</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">277.79</td>
    <td style="white-space: nowrap; text-align: right">3.60 ms</td>
    <td style="white-space: nowrap; text-align: right">±8.07%</td>
    <td style="white-space: nowrap; text-align: right">3.59 ms</td>
    <td style="white-space: nowrap; text-align: right">4.42 ms</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">204.31</td>
    <td style="white-space: nowrap; text-align: right">4.89 ms</td>
    <td style="white-space: nowrap; text-align: right">±5.98%</td>
    <td style="white-space: nowrap; text-align: right">4.95 ms</td>
    <td style="white-space: nowrap; text-align: right">5.46 ms</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">61.68</td>
    <td style="white-space: nowrap; text-align: right">16.21 ms</td>
    <td style="white-space: nowrap; text-align: right">±12.93%</td>
    <td style="white-space: nowrap; text-align: right">15.53 ms</td>
    <td style="white-space: nowrap; text-align: right">21.73 ms</td>
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
    <td style="white-space: nowrap;text-align: right">14300.20</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">277.79</td>
    <td style="white-space: nowrap; text-align: right">51.48x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">204.31</td>
    <td style="white-space: nowrap; text-align: right">69.99x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">61.68</td>
    <td style="white-space: nowrap; text-align: right">231.84x</td>
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
    <td style="white-space: nowrap; text-align: right">7102.15</td>
    <td style="white-space: nowrap; text-align: right">0.141 ms</td>
    <td style="white-space: nowrap; text-align: right">±86.67%</td>
    <td style="white-space: nowrap; text-align: right">0.0714 ms</td>
    <td style="white-space: nowrap; text-align: right">0.42 ms</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">137.08</td>
    <td style="white-space: nowrap; text-align: right">7.30 ms</td>
    <td style="white-space: nowrap; text-align: right">±4.44%</td>
    <td style="white-space: nowrap; text-align: right">7.38 ms</td>
    <td style="white-space: nowrap; text-align: right">7.77 ms</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">101.21</td>
    <td style="white-space: nowrap; text-align: right">9.88 ms</td>
    <td style="white-space: nowrap; text-align: right">±2.11%</td>
    <td style="white-space: nowrap; text-align: right">9.85 ms</td>
    <td style="white-space: nowrap; text-align: right">10.27 ms</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">28.45</td>
    <td style="white-space: nowrap; text-align: right">35.15 ms</td>
    <td style="white-space: nowrap; text-align: right">±1.97%</td>
    <td style="white-space: nowrap; text-align: right">34.81 ms</td>
    <td style="white-space: nowrap; text-align: right">36.38 ms</td>
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
    <td style="white-space: nowrap;text-align: right">7102.15</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">137.08</td>
    <td style="white-space: nowrap; text-align: right">51.81x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">101.21</td>
    <td style="white-space: nowrap; text-align: right">70.17x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">28.45</td>
    <td style="white-space: nowrap; text-align: right">249.66x</td>
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
    <td style="white-space: nowrap; text-align: right">3037.68</td>
    <td style="white-space: nowrap; text-align: right">0.33 ms</td>
    <td style="white-space: nowrap; text-align: right">±62.98%</td>
    <td style="white-space: nowrap; text-align: right">0.27 ms</td>
    <td style="white-space: nowrap; text-align: right">0.66 ms</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">64.05</td>
    <td style="white-space: nowrap; text-align: right">15.61 ms</td>
    <td style="white-space: nowrap; text-align: right">±2.26%</td>
    <td style="white-space: nowrap; text-align: right">15.65 ms</td>
    <td style="white-space: nowrap; text-align: right">16.23 ms</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">38.50</td>
    <td style="white-space: nowrap; text-align: right">25.97 ms</td>
    <td style="white-space: nowrap; text-align: right">±5.61%</td>
    <td style="white-space: nowrap; text-align: right">25.37 ms</td>
    <td style="white-space: nowrap; text-align: right">28.94 ms</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">15.02</td>
    <td style="white-space: nowrap; text-align: right">66.58 ms</td>
    <td style="white-space: nowrap; text-align: right">±1.52%</td>
    <td style="white-space: nowrap; text-align: right">67.12 ms</td>
    <td style="white-space: nowrap; text-align: right">67.20 ms</td>
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
    <td style="white-space: nowrap;text-align: right">3037.68</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">64.05</td>
    <td style="white-space: nowrap; text-align: right">47.43x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">38.50</td>
    <td style="white-space: nowrap; text-align: right">78.89x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">15.02</td>
    <td style="white-space: nowrap; text-align: right">202.24x</td>
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
    <td style="white-space: nowrap; text-align: right">828.42</td>
    <td style="white-space: nowrap; text-align: right">1.21 ms</td>
    <td style="white-space: nowrap; text-align: right">±53.36%</td>
    <td style="white-space: nowrap; text-align: right">1.22 ms</td>
    <td style="white-space: nowrap; text-align: right">2.04 ms</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">30.71</td>
    <td style="white-space: nowrap; text-align: right">32.56 ms</td>
    <td style="white-space: nowrap; text-align: right">±0.94%</td>
    <td style="white-space: nowrap; text-align: right">32.69 ms</td>
    <td style="white-space: nowrap; text-align: right">32.74 ms</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">12.05</td>
    <td style="white-space: nowrap; text-align: right">82.97 ms</td>
    <td style="white-space: nowrap; text-align: right">±1.77%</td>
    <td style="white-space: nowrap; text-align: right">83.32 ms</td>
    <td style="white-space: nowrap; text-align: right">84.24 ms</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">7.20</td>
    <td style="white-space: nowrap; text-align: right">138.86 ms</td>
    <td style="white-space: nowrap; text-align: right">±0.52%</td>
    <td style="white-space: nowrap; text-align: right">138.86 ms</td>
    <td style="white-space: nowrap; text-align: right">139.37 ms</td>
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
    <td style="white-space: nowrap;text-align: right">828.42</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">30.71</td>
    <td style="white-space: nowrap; text-align: right">26.97x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">12.05</td>
    <td style="white-space: nowrap; text-align: right">68.74x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">7.20</td>
    <td style="white-space: nowrap; text-align: right">115.04x</td>
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
    <td style="white-space: nowrap; text-align: right">831.03</td>
    <td style="white-space: nowrap; text-align: right">1.20 ms</td>
    <td style="white-space: nowrap; text-align: right">±84.47%</td>
    <td style="white-space: nowrap; text-align: right">0.65 ms</td>
    <td style="white-space: nowrap; text-align: right">2.38 ms</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">13.93</td>
    <td style="white-space: nowrap; text-align: right">71.77 ms</td>
    <td style="white-space: nowrap; text-align: right">±1.74%</td>
    <td style="white-space: nowrap; text-align: right">71.77 ms</td>
    <td style="white-space: nowrap; text-align: right">72.66 ms</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">6.15</td>
    <td style="white-space: nowrap; text-align: right">162.62 ms</td>
    <td style="white-space: nowrap; text-align: right">±0.00%</td>
    <td style="white-space: nowrap; text-align: right">162.62 ms</td>
    <td style="white-space: nowrap; text-align: right">162.62 ms</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">3.54</td>
    <td style="white-space: nowrap; text-align: right">282.88 ms</td>
    <td style="white-space: nowrap; text-align: right">±0.00%</td>
    <td style="white-space: nowrap; text-align: right">282.88 ms</td>
    <td style="white-space: nowrap; text-align: right">282.88 ms</td>
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
    <td style="white-space: nowrap;text-align: right">831.03</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">13.93</td>
    <td style="white-space: nowrap; text-align: right">59.65x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">6.15</td>
    <td style="white-space: nowrap; text-align: right">135.14x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">3.54</td>
    <td style="white-space: nowrap; text-align: right">235.09x</td>
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
    <td style="white-space: nowrap; text-align: right">167.15</td>
    <td style="white-space: nowrap; text-align: right">5.98 ms</td>
    <td style="white-space: nowrap; text-align: right">±1.26%</td>
    <td style="white-space: nowrap; text-align: right">5.98 ms</td>
    <td style="white-space: nowrap; text-align: right">6.04 ms</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">7.26</td>
    <td style="white-space: nowrap; text-align: right">137.67 ms</td>
    <td style="white-space: nowrap; text-align: right">±0.00%</td>
    <td style="white-space: nowrap; text-align: right">137.67 ms</td>
    <td style="white-space: nowrap; text-align: right">137.67 ms</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">2.38</td>
    <td style="white-space: nowrap; text-align: right">420.80 ms</td>
    <td style="white-space: nowrap; text-align: right">±0.00%</td>
    <td style="white-space: nowrap; text-align: right">420.80 ms</td>
    <td style="white-space: nowrap; text-align: right">420.80 ms</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">1.76</td>
    <td style="white-space: nowrap; text-align: right">567.48 ms</td>
    <td style="white-space: nowrap; text-align: right">±0.00%</td>
    <td style="white-space: nowrap; text-align: right">567.48 ms</td>
    <td style="white-space: nowrap; text-align: right">567.48 ms</td>
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
    <td style="white-space: nowrap;text-align: right">167.15</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">7.26</td>
    <td style="white-space: nowrap; text-align: right">23.01x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">2.38</td>
    <td style="white-space: nowrap; text-align: right">70.34x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">1.76</td>
    <td style="white-space: nowrap; text-align: right">94.86x</td>
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
    <td style="white-space: nowrap; text-align: right">99.16</td>
    <td style="white-space: nowrap; text-align: right">10.09 ms</td>
    <td style="white-space: nowrap; text-align: right">±0.00%</td>
    <td style="white-space: nowrap; text-align: right">10.09 ms</td>
    <td style="white-space: nowrap; text-align: right">10.09 ms</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">3.64</td>
    <td style="white-space: nowrap; text-align: right">274.91 ms</td>
    <td style="white-space: nowrap; text-align: right">±0.00%</td>
    <td style="white-space: nowrap; text-align: right">274.91 ms</td>
    <td style="white-space: nowrap; text-align: right">274.91 ms</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">1.11</td>
    <td style="white-space: nowrap; text-align: right">898.03 ms</td>
    <td style="white-space: nowrap; text-align: right">±0.00%</td>
    <td style="white-space: nowrap; text-align: right">898.03 ms</td>
    <td style="white-space: nowrap; text-align: right">898.03 ms</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.87</td>
    <td style="white-space: nowrap; text-align: right">1146.97 ms</td>
    <td style="white-space: nowrap; text-align: right">±0.00%</td>
    <td style="white-space: nowrap; text-align: right">1146.97 ms</td>
    <td style="white-space: nowrap; text-align: right">1146.97 ms</td>
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
    <td style="white-space: nowrap;text-align: right">99.16</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">3.64</td>
    <td style="white-space: nowrap; text-align: right">27.26x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">1.11</td>
    <td style="white-space: nowrap; text-align: right">89.05x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.87</td>
    <td style="white-space: nowrap; text-align: right">113.73x</td>
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
    <td style="white-space: nowrap; text-align: right">218.34</td>
    <td style="white-space: nowrap; text-align: right">0.00458 s</td>
    <td style="white-space: nowrap; text-align: right">±0.00%</td>
    <td style="white-space: nowrap; text-align: right">0.00458 s</td>
    <td style="white-space: nowrap; text-align: right">0.00458 s</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">1.33</td>
    <td style="white-space: nowrap; text-align: right">0.75 s</td>
    <td style="white-space: nowrap; text-align: right">±0.00%</td>
    <td style="white-space: nowrap; text-align: right">0.75 s</td>
    <td style="white-space: nowrap; text-align: right">0.75 s</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">0.47</td>
    <td style="white-space: nowrap; text-align: right">2.13 s</td>
    <td style="white-space: nowrap; text-align: right">±0.00%</td>
    <td style="white-space: nowrap; text-align: right">2.13 s</td>
    <td style="white-space: nowrap; text-align: right">2.13 s</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.44</td>
    <td style="white-space: nowrap; text-align: right">2.29 s</td>
    <td style="white-space: nowrap; text-align: right">±0.00%</td>
    <td style="white-space: nowrap; text-align: right">2.29 s</td>
    <td style="white-space: nowrap; text-align: right">2.29 s</td>
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
    <td style="white-space: nowrap;text-align: right">218.34</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">1.33</td>
    <td style="white-space: nowrap; text-align: right">164.41x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">0.47</td>
    <td style="white-space: nowrap; text-align: right">465.54x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.concat/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.44</td>
    <td style="white-space: nowrap; text-align: right">500.01x</td>
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

