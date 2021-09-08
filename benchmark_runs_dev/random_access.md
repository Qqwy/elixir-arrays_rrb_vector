
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
    <td style="white-space: nowrap; text-align: right">5.80 M</td>
    <td style="white-space: nowrap; text-align: right">172.27 ns</td>
    <td style="white-space: nowrap; text-align: right">±608.72%</td>
    <td style="white-space: nowrap; text-align: right">138 ns</td>
    <td style="white-space: nowrap; text-align: right">286.36 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">5.07 M</td>
    <td style="white-space: nowrap; text-align: right">197.06 ns</td>
    <td style="white-space: nowrap; text-align: right">±585.47%</td>
    <td style="white-space: nowrap; text-align: right">142 ns</td>
    <td style="white-space: nowrap; text-align: right">622 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">4.64 M</td>
    <td style="white-space: nowrap; text-align: right">215.30 ns</td>
    <td style="white-space: nowrap; text-align: right">±1021.32%</td>
    <td style="white-space: nowrap; text-align: right">150 ns</td>
    <td style="white-space: nowrap; text-align: right">320.61 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.31 M</td>
    <td style="white-space: nowrap; text-align: right">3265.20 ns</td>
    <td style="white-space: nowrap; text-align: right">±105.24%</td>
    <td style="white-space: nowrap; text-align: right">2920 ns</td>
    <td style="white-space: nowrap; text-align: right">19464.64 ns</td>
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
    <td style="white-space: nowrap;text-align: right">5.80 M</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">5.07 M</td>
    <td style="white-space: nowrap; text-align: right">1.14x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">4.64 M</td>
    <td style="white-space: nowrap; text-align: right">1.25x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.31 M</td>
    <td style="white-space: nowrap; text-align: right">18.95x</td>
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
    <td style="white-space: nowrap">Arrays.get/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">5.64 M</td>
    <td style="white-space: nowrap; text-align: right">177.25 ns</td>
    <td style="white-space: nowrap; text-align: right">±620.22%</td>
    <td style="white-space: nowrap; text-align: right">136 ns</td>
    <td style="white-space: nowrap; text-align: right">280.66 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">5.04 M</td>
    <td style="white-space: nowrap; text-align: right">198.49 ns</td>
    <td style="white-space: nowrap; text-align: right">±764.89%</td>
    <td style="white-space: nowrap; text-align: right">140 ns</td>
    <td style="white-space: nowrap; text-align: right">438.74 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Enum.fetch/2 (list)</td>
    <td style="white-space: nowrap; text-align: right">4.53 M</td>
    <td style="white-space: nowrap; text-align: right">220.84 ns</td>
    <td style="white-space: nowrap; text-align: right">±452.03%</td>
    <td style="white-space: nowrap; text-align: right">182 ns</td>
    <td style="white-space: nowrap; text-align: right">418.95 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.31 M</td>
    <td style="white-space: nowrap; text-align: right">3278.01 ns</td>
    <td style="white-space: nowrap; text-align: right">±92.47%</td>
    <td style="white-space: nowrap; text-align: right">2923 ns</td>
    <td style="white-space: nowrap; text-align: right">20022.40 ns</td>
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
    <td style="white-space: nowrap;text-align: right">5.64 M</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">5.04 M</td>
    <td style="white-space: nowrap; text-align: right">1.12x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Enum.fetch/2 (list)</td>
    <td style="white-space: nowrap; text-align: right">4.53 M</td>
    <td style="white-space: nowrap; text-align: right">1.25x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.31 M</td>
    <td style="white-space: nowrap; text-align: right">18.49x</td>
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
    <td style="white-space: nowrap; text-align: right">5.52 M</td>
    <td style="white-space: nowrap; text-align: right">181.13 ns</td>
    <td style="white-space: nowrap; text-align: right">±639.22%</td>
    <td style="white-space: nowrap; text-align: right">141 ns</td>
    <td style="white-space: nowrap; text-align: right">243.42 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">4.59 M</td>
    <td style="white-space: nowrap; text-align: right">217.79 ns</td>
    <td style="white-space: nowrap; text-align: right">±564.56%</td>
    <td style="white-space: nowrap; text-align: right">147 ns</td>
    <td style="white-space: nowrap; text-align: right">744.28 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Enum.fetch/2 (list)</td>
    <td style="white-space: nowrap; text-align: right">3.30 M</td>
    <td style="white-space: nowrap; text-align: right">303.10 ns</td>
    <td style="white-space: nowrap; text-align: right">±334.93%</td>
    <td style="white-space: nowrap; text-align: right">259 ns</td>
    <td style="white-space: nowrap; text-align: right">654.07 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.31 M</td>
    <td style="white-space: nowrap; text-align: right">3266.05 ns</td>
    <td style="white-space: nowrap; text-align: right">±100.76%</td>
    <td style="white-space: nowrap; text-align: right">2956 ns</td>
    <td style="white-space: nowrap; text-align: right">19461.73 ns</td>
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
    <td style="white-space: nowrap;text-align: right">5.52 M</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">4.59 M</td>
    <td style="white-space: nowrap; text-align: right">1.2x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Enum.fetch/2 (list)</td>
    <td style="white-space: nowrap; text-align: right">3.30 M</td>
    <td style="white-space: nowrap; text-align: right">1.67x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.31 M</td>
    <td style="white-space: nowrap; text-align: right">18.03x</td>
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
    <td style="white-space: nowrap">Arrays.get/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">4.51 M</td>
    <td style="white-space: nowrap; text-align: right">221.53 ns</td>
    <td style="white-space: nowrap; text-align: right">±890.43%</td>
    <td style="white-space: nowrap; text-align: right">160 ns</td>
    <td style="white-space: nowrap; text-align: right">400.21 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">4.10 M</td>
    <td style="white-space: nowrap; text-align: right">244.19 ns</td>
    <td style="white-space: nowrap; text-align: right">±639.48%</td>
    <td style="white-space: nowrap; text-align: right">152 ns</td>
    <td style="white-space: nowrap; text-align: right">858.10 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Enum.fetch/2 (list)</td>
    <td style="white-space: nowrap; text-align: right">2.00 M</td>
    <td style="white-space: nowrap; text-align: right">499.96 ns</td>
    <td style="white-space: nowrap; text-align: right">±226.40%</td>
    <td style="white-space: nowrap; text-align: right">447 ns</td>
    <td style="white-space: nowrap; text-align: right">982.64 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.29 M</td>
    <td style="white-space: nowrap; text-align: right">3435.44 ns</td>
    <td style="white-space: nowrap; text-align: right">±125.65%</td>
    <td style="white-space: nowrap; text-align: right">3111 ns</td>
    <td style="white-space: nowrap; text-align: right">19621.56 ns</td>
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
    <td style="white-space: nowrap;text-align: right">4.51 M</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">4.10 M</td>
    <td style="white-space: nowrap; text-align: right">1.1x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Enum.fetch/2 (list)</td>
    <td style="white-space: nowrap; text-align: right">2.00 M</td>
    <td style="white-space: nowrap; text-align: right">2.26x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.29 M</td>
    <td style="white-space: nowrap; text-align: right">15.51x</td>
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
    <td style="white-space: nowrap">Arrays.get/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">4.72 M</td>
    <td style="white-space: nowrap; text-align: right">212.08 ns</td>
    <td style="white-space: nowrap; text-align: right">±635.99%</td>
    <td style="white-space: nowrap; text-align: right">160 ns</td>
    <td style="white-space: nowrap; text-align: right">293.80 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">4.67 M</td>
    <td style="white-space: nowrap; text-align: right">214.22 ns</td>
    <td style="white-space: nowrap; text-align: right">±693.32%</td>
    <td style="white-space: nowrap; text-align: right">161 ns</td>
    <td style="white-space: nowrap; text-align: right">322.74 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Enum.fetch/2 (list)</td>
    <td style="white-space: nowrap; text-align: right">1.16 M</td>
    <td style="white-space: nowrap; text-align: right">865.15 ns</td>
    <td style="white-space: nowrap; text-align: right">±189.65%</td>
    <td style="white-space: nowrap; text-align: right">788 ns</td>
    <td style="white-space: nowrap; text-align: right">1686 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.29 M</td>
    <td style="white-space: nowrap; text-align: right">3495.39 ns</td>
    <td style="white-space: nowrap; text-align: right">±89.66%</td>
    <td style="white-space: nowrap; text-align: right">3108 ns</td>
    <td style="white-space: nowrap; text-align: right">20969.00 ns</td>
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
    <td style="white-space: nowrap;text-align: right">4.72 M</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">4.67 M</td>
    <td style="white-space: nowrap; text-align: right">1.01x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Enum.fetch/2 (list)</td>
    <td style="white-space: nowrap; text-align: right">1.16 M</td>
    <td style="white-space: nowrap; text-align: right">4.08x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.29 M</td>
    <td style="white-space: nowrap; text-align: right">16.48x</td>
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
    <td style="white-space: nowrap">Arrays.get/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">4.43 M</td>
    <td style="white-space: nowrap; text-align: right">0.23 μs</td>
    <td style="white-space: nowrap; text-align: right">±641.52%</td>
    <td style="white-space: nowrap; text-align: right">0.160 μs</td>
    <td style="white-space: nowrap; text-align: right">0.28 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">4.42 M</td>
    <td style="white-space: nowrap; text-align: right">0.23 μs</td>
    <td style="white-space: nowrap; text-align: right">±646.47%</td>
    <td style="white-space: nowrap; text-align: right">0.160 μs</td>
    <td style="white-space: nowrap; text-align: right">0.81 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Enum.fetch/2 (list)</td>
    <td style="white-space: nowrap; text-align: right">0.60 M</td>
    <td style="white-space: nowrap; text-align: right">1.68 μs</td>
    <td style="white-space: nowrap; text-align: right">±122.11%</td>
    <td style="white-space: nowrap; text-align: right">1.50 μs</td>
    <td style="white-space: nowrap; text-align: right">4.48 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.28 M</td>
    <td style="white-space: nowrap; text-align: right">3.54 μs</td>
    <td style="white-space: nowrap; text-align: right">±97.06%</td>
    <td style="white-space: nowrap; text-align: right">3.16 μs</td>
    <td style="white-space: nowrap; text-align: right">21.00 μs</td>
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
    <td style="white-space: nowrap;text-align: right">4.43 M</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">4.42 M</td>
    <td style="white-space: nowrap; text-align: right">1.0x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Enum.fetch/2 (list)</td>
    <td style="white-space: nowrap; text-align: right">0.60 M</td>
    <td style="white-space: nowrap; text-align: right">7.43x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.28 M</td>
    <td style="white-space: nowrap; text-align: right">15.7x</td>
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
    <td style="white-space: nowrap">Arrays.get/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">4.60 M</td>
    <td style="white-space: nowrap; text-align: right">0.22 μs</td>
    <td style="white-space: nowrap; text-align: right">±519.32%</td>
    <td style="white-space: nowrap; text-align: right">0.170 μs</td>
    <td style="white-space: nowrap; text-align: right">0.30 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">4.41 M</td>
    <td style="white-space: nowrap; text-align: right">0.23 μs</td>
    <td style="white-space: nowrap; text-align: right">±731.77%</td>
    <td style="white-space: nowrap; text-align: right">0.168 μs</td>
    <td style="white-space: nowrap; text-align: right">0.33 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Enum.fetch/2 (list)</td>
    <td style="white-space: nowrap; text-align: right">0.31 M</td>
    <td style="white-space: nowrap; text-align: right">3.26 μs</td>
    <td style="white-space: nowrap; text-align: right">±86.57%</td>
    <td style="white-space: nowrap; text-align: right">3.03 μs</td>
    <td style="white-space: nowrap; text-align: right">19.15 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.27 M</td>
    <td style="white-space: nowrap; text-align: right">3.68 μs</td>
    <td style="white-space: nowrap; text-align: right">±90.08%</td>
    <td style="white-space: nowrap; text-align: right">3.29 μs</td>
    <td style="white-space: nowrap; text-align: right">21.35 μs</td>
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
    <td style="white-space: nowrap;text-align: right">4.60 M</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">4.41 M</td>
    <td style="white-space: nowrap; text-align: right">1.04x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Enum.fetch/2 (list)</td>
    <td style="white-space: nowrap; text-align: right">0.31 M</td>
    <td style="white-space: nowrap; text-align: right">14.98x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.27 M</td>
    <td style="white-space: nowrap; text-align: right">16.9x</td>
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
    <td style="white-space: nowrap">Arrays.get/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">4.02 M</td>
    <td style="white-space: nowrap; text-align: right">0.25 μs</td>
    <td style="white-space: nowrap; text-align: right">±850.37%</td>
    <td style="white-space: nowrap; text-align: right">0.178 μs</td>
    <td style="white-space: nowrap; text-align: right">0.31 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">3.63 M</td>
    <td style="white-space: nowrap; text-align: right">0.28 μs</td>
    <td style="white-space: nowrap; text-align: right">±1231.53%</td>
    <td style="white-space: nowrap; text-align: right">0.183 μs</td>
    <td style="white-space: nowrap; text-align: right">0.51 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.28 M</td>
    <td style="white-space: nowrap; text-align: right">3.54 μs</td>
    <td style="white-space: nowrap; text-align: right">±98.60%</td>
    <td style="white-space: nowrap; text-align: right">3.18 μs</td>
    <td style="white-space: nowrap; text-align: right">19.99 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Enum.fetch/2 (list)</td>
    <td style="white-space: nowrap; text-align: right">0.158 M</td>
    <td style="white-space: nowrap; text-align: right">6.32 μs</td>
    <td style="white-space: nowrap; text-align: right">±74.58%</td>
    <td style="white-space: nowrap; text-align: right">5.96 μs</td>
    <td style="white-space: nowrap; text-align: right">26.03 μs</td>
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
    <td style="white-space: nowrap;text-align: right">4.02 M</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">3.63 M</td>
    <td style="white-space: nowrap; text-align: right">1.11x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.28 M</td>
    <td style="white-space: nowrap; text-align: right">14.24x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Enum.fetch/2 (list)</td>
    <td style="white-space: nowrap; text-align: right">0.158 M</td>
    <td style="white-space: nowrap; text-align: right">25.4x</td>
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
    <td style="white-space: nowrap">Arrays.get/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">4.32 M</td>
    <td style="white-space: nowrap; text-align: right">0.23 μs</td>
    <td style="white-space: nowrap; text-align: right">±173.55%</td>
    <td style="white-space: nowrap; text-align: right">0.22 μs</td>
    <td style="white-space: nowrap; text-align: right">0.34 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">3.96 M</td>
    <td style="white-space: nowrap; text-align: right">0.25 μs</td>
    <td style="white-space: nowrap; text-align: right">±307.30%</td>
    <td style="white-space: nowrap; text-align: right">0.20 μs</td>
    <td style="white-space: nowrap; text-align: right">0.71 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.26 M</td>
    <td style="white-space: nowrap; text-align: right">3.88 μs</td>
    <td style="white-space: nowrap; text-align: right">±81.11%</td>
    <td style="white-space: nowrap; text-align: right">3.54 μs</td>
    <td style="white-space: nowrap; text-align: right">20.45 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Enum.fetch/2 (list)</td>
    <td style="white-space: nowrap; text-align: right">0.0809 M</td>
    <td style="white-space: nowrap; text-align: right">12.36 μs</td>
    <td style="white-space: nowrap; text-align: right">±94.72%</td>
    <td style="white-space: nowrap; text-align: right">11.73 μs</td>
    <td style="white-space: nowrap; text-align: right">44.61 μs</td>
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
    <td style="white-space: nowrap;text-align: right">4.32 M</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">3.96 M</td>
    <td style="white-space: nowrap; text-align: right">1.09x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.26 M</td>
    <td style="white-space: nowrap; text-align: right">16.75x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Enum.fetch/2 (list)</td>
    <td style="white-space: nowrap; text-align: right">0.0809 M</td>
    <td style="white-space: nowrap; text-align: right">53.33x</td>
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
    <td style="white-space: nowrap; text-align: right">3.66 M</td>
    <td style="white-space: nowrap; text-align: right">0.27 μs</td>
    <td style="white-space: nowrap; text-align: right">±231.46%</td>
    <td style="white-space: nowrap; text-align: right">0.22 μs</td>
    <td style="white-space: nowrap; text-align: right">0.88 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">3.58 M</td>
    <td style="white-space: nowrap; text-align: right">0.28 μs</td>
    <td style="white-space: nowrap; text-align: right">±323.50%</td>
    <td style="white-space: nowrap; text-align: right">0.23 μs</td>
    <td style="white-space: nowrap; text-align: right">0.38 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.26 M</td>
    <td style="white-space: nowrap; text-align: right">3.81 μs</td>
    <td style="white-space: nowrap; text-align: right">±79.98%</td>
    <td style="white-space: nowrap; text-align: right">3.50 μs</td>
    <td style="white-space: nowrap; text-align: right">20.06 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Enum.fetch/2 (list)</td>
    <td style="white-space: nowrap; text-align: right">0.0437 M</td>
    <td style="white-space: nowrap; text-align: right">22.89 μs</td>
    <td style="white-space: nowrap; text-align: right">±63.62%</td>
    <td style="white-space: nowrap; text-align: right">22.28 μs</td>
    <td style="white-space: nowrap; text-align: right">64.03 μs</td>
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
    <td style="white-space: nowrap;text-align: right">3.66 M</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">3.58 M</td>
    <td style="white-space: nowrap; text-align: right">1.02x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">0.26 M</td>
    <td style="white-space: nowrap; text-align: right">13.95x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Enum.fetch/2 (list)</td>
    <td style="white-space: nowrap; text-align: right">0.0437 M</td>
    <td style="white-space: nowrap; text-align: right">83.76x</td>
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
    <td style="white-space: nowrap">Arrays.get/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">652.87 K</td>
    <td style="white-space: nowrap; text-align: right">1.53 μs</td>
    <td style="white-space: nowrap; text-align: right">±1694.87%</td>
    <td style="white-space: nowrap; text-align: right">0.24 μs</td>
    <td style="white-space: nowrap; text-align: right">1.06 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">606.82 K</td>
    <td style="white-space: nowrap; text-align: right">1.65 μs</td>
    <td style="white-space: nowrap; text-align: right">±1681.84%</td>
    <td style="white-space: nowrap; text-align: right">0.25 μs</td>
    <td style="white-space: nowrap; text-align: right">1.31 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">253.51 K</td>
    <td style="white-space: nowrap; text-align: right">3.94 μs</td>
    <td style="white-space: nowrap; text-align: right">±138.73%</td>
    <td style="white-space: nowrap; text-align: right">3.49 μs</td>
    <td style="white-space: nowrap; text-align: right">20.71 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Enum.fetch/2 (list)</td>
    <td style="white-space: nowrap; text-align: right">21.30 K</td>
    <td style="white-space: nowrap; text-align: right">46.95 μs</td>
    <td style="white-space: nowrap; text-align: right">±64.25%</td>
    <td style="white-space: nowrap; text-align: right">45.03 μs</td>
    <td style="white-space: nowrap; text-align: right">143.98 μs</td>
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
    <td style="white-space: nowrap;text-align: right">652.87 K</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">606.82 K</td>
    <td style="white-space: nowrap; text-align: right">1.08x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">253.51 K</td>
    <td style="white-space: nowrap; text-align: right">2.58x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Enum.fetch/2 (list)</td>
    <td style="white-space: nowrap; text-align: right">21.30 K</td>
    <td style="white-space: nowrap; text-align: right">30.65x</td>
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
    <td style="white-space: nowrap">Arrays.get/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">575.62 K</td>
    <td style="white-space: nowrap; text-align: right">1.74 μs</td>
    <td style="white-space: nowrap; text-align: right">±125.10%</td>
    <td style="white-space: nowrap; text-align: right">1.41 μs</td>
    <td style="white-space: nowrap; text-align: right">17.17 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">475.48 K</td>
    <td style="white-space: nowrap; text-align: right">2.10 μs</td>
    <td style="white-space: nowrap; text-align: right">±117.79%</td>
    <td style="white-space: nowrap; text-align: right">1.66 μs</td>
    <td style="white-space: nowrap; text-align: right">14.92 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">253.81 K</td>
    <td style="white-space: nowrap; text-align: right">3.94 μs</td>
    <td style="white-space: nowrap; text-align: right">±112.74%</td>
    <td style="white-space: nowrap; text-align: right">3.53 μs</td>
    <td style="white-space: nowrap; text-align: right">20.59 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Enum.fetch/2 (list)</td>
    <td style="white-space: nowrap; text-align: right">10.28 K</td>
    <td style="white-space: nowrap; text-align: right">97.31 μs</td>
    <td style="white-space: nowrap; text-align: right">±75.37%</td>
    <td style="white-space: nowrap; text-align: right">92.31 μs</td>
    <td style="white-space: nowrap; text-align: right">249.48 μs</td>
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
    <td style="white-space: nowrap;text-align: right">575.62 K</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">475.48 K</td>
    <td style="white-space: nowrap; text-align: right">1.21x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">253.81 K</td>
    <td style="white-space: nowrap; text-align: right">2.27x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Enum.fetch/2 (list)</td>
    <td style="white-space: nowrap; text-align: right">10.28 K</td>
    <td style="white-space: nowrap; text-align: right">56.01x</td>
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
    <td style="white-space: nowrap">Arrays.get/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">350.51 K</td>
    <td style="white-space: nowrap; text-align: right">2.85 μs</td>
    <td style="white-space: nowrap; text-align: right">±61.51%</td>
    <td style="white-space: nowrap; text-align: right">2.57 μs</td>
    <td style="white-space: nowrap; text-align: right">16.10 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">275.48 K</td>
    <td style="white-space: nowrap; text-align: right">3.63 μs</td>
    <td style="white-space: nowrap; text-align: right">±54.80%</td>
    <td style="white-space: nowrap; text-align: right">3.36 μs</td>
    <td style="white-space: nowrap; text-align: right">17.65 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">252.66 K</td>
    <td style="white-space: nowrap; text-align: right">3.96 μs</td>
    <td style="white-space: nowrap; text-align: right">±82.36%</td>
    <td style="white-space: nowrap; text-align: right">3.55 μs</td>
    <td style="white-space: nowrap; text-align: right">21.84 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Enum.fetch/2 (list)</td>
    <td style="white-space: nowrap; text-align: right">4.35 K</td>
    <td style="white-space: nowrap; text-align: right">230.10 μs</td>
    <td style="white-space: nowrap; text-align: right">±59.85%</td>
    <td style="white-space: nowrap; text-align: right">226.78 μs</td>
    <td style="white-space: nowrap; text-align: right">568.98 μs</td>
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
    <td style="white-space: nowrap;text-align: right">350.51 K</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">275.48 K</td>
    <td style="white-space: nowrap; text-align: right">1.27x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">252.66 K</td>
    <td style="white-space: nowrap; text-align: right">1.39x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Enum.fetch/2 (list)</td>
    <td style="white-space: nowrap; text-align: right">4.35 K</td>
    <td style="white-space: nowrap; text-align: right">80.65x</td>
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
    <td style="white-space: nowrap">Arrays.get/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">350.05 K</td>
    <td style="white-space: nowrap; text-align: right">2.86 μs</td>
    <td style="white-space: nowrap; text-align: right">±70.47%</td>
    <td style="white-space: nowrap; text-align: right">2.58 μs</td>
    <td style="white-space: nowrap; text-align: right">22.36 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">348.64 K</td>
    <td style="white-space: nowrap; text-align: right">2.87 μs</td>
    <td style="white-space: nowrap; text-align: right">±65.53%</td>
    <td style="white-space: nowrap; text-align: right">2.66 μs</td>
    <td style="white-space: nowrap; text-align: right">20.96 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">246.65 K</td>
    <td style="white-space: nowrap; text-align: right">4.05 μs</td>
    <td style="white-space: nowrap; text-align: right">±121.57%</td>
    <td style="white-space: nowrap; text-align: right">3.61 μs</td>
    <td style="white-space: nowrap; text-align: right">21.51 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Enum.fetch/2 (list)</td>
    <td style="white-space: nowrap; text-align: right">1.82 K</td>
    <td style="white-space: nowrap; text-align: right">549.87 μs</td>
    <td style="white-space: nowrap; text-align: right">±52.67%</td>
    <td style="white-space: nowrap; text-align: right">588.19 μs</td>
    <td style="white-space: nowrap; text-align: right">1044.37 μs</td>
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
    <td style="white-space: nowrap;text-align: right">350.05 K</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">348.64 K</td>
    <td style="white-space: nowrap; text-align: right">1.0x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">246.65 K</td>
    <td style="white-space: nowrap; text-align: right">1.42x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Enum.fetch/2 (list)</td>
    <td style="white-space: nowrap; text-align: right">1.82 K</td>
    <td style="white-space: nowrap; text-align: right">192.48x</td>
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
    <td style="white-space: nowrap">Arrays.get/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">369.18 K</td>
    <td style="white-space: nowrap; text-align: right">2.71 μs</td>
    <td style="white-space: nowrap; text-align: right">±19.52%</td>
    <td style="white-space: nowrap; text-align: right">2.59 μs</td>
    <td style="white-space: nowrap; text-align: right">4.40 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">312.33 K</td>
    <td style="white-space: nowrap; text-align: right">3.20 μs</td>
    <td style="white-space: nowrap; text-align: right">±101.94%</td>
    <td style="white-space: nowrap; text-align: right">2.48 μs</td>
    <td style="white-space: nowrap; text-align: right">21.23 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">233.02 K</td>
    <td style="white-space: nowrap; text-align: right">4.29 μs</td>
    <td style="white-space: nowrap; text-align: right">±115.38%</td>
    <td style="white-space: nowrap; text-align: right">3.81 μs</td>
    <td style="white-space: nowrap; text-align: right">21.94 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Enum.fetch/2 (list)</td>
    <td style="white-space: nowrap; text-align: right">0.97 K</td>
    <td style="white-space: nowrap; text-align: right">1032.24 μs</td>
    <td style="white-space: nowrap; text-align: right">±55.15%</td>
    <td style="white-space: nowrap; text-align: right">1050.62 μs</td>
    <td style="white-space: nowrap; text-align: right">2074.17 μs</td>
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
    <td style="white-space: nowrap;text-align: right">369.18 K</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">312.33 K</td>
    <td style="white-space: nowrap; text-align: right">1.18x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">233.02 K</td>
    <td style="white-space: nowrap; text-align: right">1.58x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Enum.fetch/2 (list)</td>
    <td style="white-space: nowrap; text-align: right">0.97 K</td>
    <td style="white-space: nowrap; text-align: right">381.08x</td>
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
    <td style="white-space: nowrap">Arrays.get/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">274.84 K</td>
    <td style="white-space: nowrap; text-align: right">3.64 μs</td>
    <td style="white-space: nowrap; text-align: right">±79.34%</td>
    <td style="white-space: nowrap; text-align: right">2.92 μs</td>
    <td style="white-space: nowrap; text-align: right">16.09 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">223.78 K</td>
    <td style="white-space: nowrap; text-align: right">4.47 μs</td>
    <td style="white-space: nowrap; text-align: right">±170.64%</td>
    <td style="white-space: nowrap; text-align: right">3.78 μs</td>
    <td style="white-space: nowrap; text-align: right">22.95 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">199.12 K</td>
    <td style="white-space: nowrap; text-align: right">5.02 μs</td>
    <td style="white-space: nowrap; text-align: right">±112.58%</td>
    <td style="white-space: nowrap; text-align: right">2.93 μs</td>
    <td style="white-space: nowrap; text-align: right">23.61 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Enum.fetch/2 (list)</td>
    <td style="white-space: nowrap; text-align: right">0.46 K</td>
    <td style="white-space: nowrap; text-align: right">2164.96 μs</td>
    <td style="white-space: nowrap; text-align: right">±51.62%</td>
    <td style="white-space: nowrap; text-align: right">2117.94 μs</td>
    <td style="white-space: nowrap; text-align: right">6095.48 μs</td>
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
    <td style="white-space: nowrap;text-align: right">274.84 K</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">223.78 K</td>
    <td style="white-space: nowrap; text-align: right">1.23x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">199.12 K</td>
    <td style="white-space: nowrap; text-align: right">1.38x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Enum.fetch/2 (list)</td>
    <td style="white-space: nowrap; text-align: right">0.46 K</td>
    <td style="white-space: nowrap; text-align: right">595.01x</td>
  </tr>

</table>



<hr/>

