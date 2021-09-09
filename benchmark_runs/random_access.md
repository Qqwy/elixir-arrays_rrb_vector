
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
    <td style="white-space: nowrap; text-align: right">5.85 M</td>
    <td style="white-space: nowrap; text-align: right">170.81 ns</td>
    <td style="white-space: nowrap; text-align: right">±603.31%</td>
    <td style="white-space: nowrap; text-align: right">127 ns</td>
    <td style="white-space: nowrap; text-align: right">255.28 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">4.82 M</td>
    <td style="white-space: nowrap; text-align: right">207.45 ns</td>
    <td style="white-space: nowrap; text-align: right">±558.40%</td>
    <td style="white-space: nowrap; text-align: right">157 ns</td>
    <td style="white-space: nowrap; text-align: right">549.45 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">4.76 M</td>
    <td style="white-space: nowrap; text-align: right">210.28 ns</td>
    <td style="white-space: nowrap; text-align: right">±558.61%</td>
    <td style="white-space: nowrap; text-align: right">159 ns</td>
    <td style="white-space: nowrap; text-align: right">514.91 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">1.34 M</td>
    <td style="white-space: nowrap; text-align: right">748.40 ns</td>
    <td style="white-space: nowrap; text-align: right">±309.72%</td>
    <td style="white-space: nowrap; text-align: right">602 ns</td>
    <td style="white-space: nowrap; text-align: right">1183.50 ns</td>
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
    <td style="white-space: nowrap;text-align: right">5.85 M</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">4.82 M</td>
    <td style="white-space: nowrap; text-align: right">1.21x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">4.76 M</td>
    <td style="white-space: nowrap; text-align: right">1.23x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">1.34 M</td>
    <td style="white-space: nowrap; text-align: right">4.38x</td>
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
    <td style="white-space: nowrap; text-align: right">4.80 M</td>
    <td style="white-space: nowrap; text-align: right">208.17 ns</td>
    <td style="white-space: nowrap; text-align: right">±547.30%</td>
    <td style="white-space: nowrap; text-align: right">163 ns</td>
    <td style="white-space: nowrap; text-align: right">291 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">4.64 M</td>
    <td style="white-space: nowrap; text-align: right">215.50 ns</td>
    <td style="white-space: nowrap; text-align: right">±612.16%</td>
    <td style="white-space: nowrap; text-align: right">164 ns</td>
    <td style="white-space: nowrap; text-align: right">289 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Enum.fetch/2 (list)</td>
    <td style="white-space: nowrap; text-align: right">4.55 M</td>
    <td style="white-space: nowrap; text-align: right">219.55 ns</td>
    <td style="white-space: nowrap; text-align: right">±511.31%</td>
    <td style="white-space: nowrap; text-align: right">175 ns</td>
    <td style="white-space: nowrap; text-align: right">512 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">1.33 M</td>
    <td style="white-space: nowrap; text-align: right">752.27 ns</td>
    <td style="white-space: nowrap; text-align: right">±310.44%</td>
    <td style="white-space: nowrap; text-align: right">610 ns</td>
    <td style="white-space: nowrap; text-align: right">1230.57 ns</td>
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
    <td style="white-space: nowrap;text-align: right">4.80 M</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">4.64 M</td>
    <td style="white-space: nowrap; text-align: right">1.04x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Enum.fetch/2 (list)</td>
    <td style="white-space: nowrap; text-align: right">4.55 M</td>
    <td style="white-space: nowrap; text-align: right">1.05x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">1.33 M</td>
    <td style="white-space: nowrap; text-align: right">3.61x</td>
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
    <td style="white-space: nowrap; text-align: right">4.50 M</td>
    <td style="white-space: nowrap; text-align: right">222.39 ns</td>
    <td style="white-space: nowrap; text-align: right">±521.28%</td>
    <td style="white-space: nowrap; text-align: right">170 ns</td>
    <td style="white-space: nowrap; text-align: right">310.37 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">4.38 M</td>
    <td style="white-space: nowrap; text-align: right">228.36 ns</td>
    <td style="white-space: nowrap; text-align: right">±547.15%</td>
    <td style="white-space: nowrap; text-align: right">172 ns</td>
    <td style="white-space: nowrap; text-align: right">334.88 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Enum.fetch/2 (list)</td>
    <td style="white-space: nowrap; text-align: right">3.14 M</td>
    <td style="white-space: nowrap; text-align: right">318.29 ns</td>
    <td style="white-space: nowrap; text-align: right">±385.32%</td>
    <td style="white-space: nowrap; text-align: right">277 ns</td>
    <td style="white-space: nowrap; text-align: right">669.47 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">1.26 M</td>
    <td style="white-space: nowrap; text-align: right">795.69 ns</td>
    <td style="white-space: nowrap; text-align: right">±330.64%</td>
    <td style="white-space: nowrap; text-align: right">649 ns</td>
    <td style="white-space: nowrap; text-align: right">1249.00 ns</td>
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
    <td style="white-space: nowrap;text-align: right">4.50 M</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">4.38 M</td>
    <td style="white-space: nowrap; text-align: right">1.03x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Enum.fetch/2 (list)</td>
    <td style="white-space: nowrap; text-align: right">3.14 M</td>
    <td style="white-space: nowrap; text-align: right">1.43x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">1.26 M</td>
    <td style="white-space: nowrap; text-align: right">3.58x</td>
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
    <td style="white-space: nowrap; text-align: right">4.33 M</td>
    <td style="white-space: nowrap; text-align: right">230.75 ns</td>
    <td style="white-space: nowrap; text-align: right">±531.95%</td>
    <td style="white-space: nowrap; text-align: right">177 ns</td>
    <td style="white-space: nowrap; text-align: right">306.63 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">4.26 M</td>
    <td style="white-space: nowrap; text-align: right">234.88 ns</td>
    <td style="white-space: nowrap; text-align: right">±550.10%</td>
    <td style="white-space: nowrap; text-align: right">180 ns</td>
    <td style="white-space: nowrap; text-align: right">305 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Enum.fetch/2 (list)</td>
    <td style="white-space: nowrap; text-align: right">1.92 M</td>
    <td style="white-space: nowrap; text-align: right">521.01 ns</td>
    <td style="white-space: nowrap; text-align: right">±308.50%</td>
    <td style="white-space: nowrap; text-align: right">450.50 ns</td>
    <td style="white-space: nowrap; text-align: right">1024.23 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">1.20 M</td>
    <td style="white-space: nowrap; text-align: right">833.98 ns</td>
    <td style="white-space: nowrap; text-align: right">±291.98%</td>
    <td style="white-space: nowrap; text-align: right">630 ns</td>
    <td style="white-space: nowrap; text-align: right">2981.80 ns</td>
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
    <td style="white-space: nowrap;text-align: right">4.33 M</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">4.26 M</td>
    <td style="white-space: nowrap; text-align: right">1.02x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Enum.fetch/2 (list)</td>
    <td style="white-space: nowrap; text-align: right">1.92 M</td>
    <td style="white-space: nowrap; text-align: right">2.26x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">1.20 M</td>
    <td style="white-space: nowrap; text-align: right">3.61x</td>
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
    <td style="white-space: nowrap; text-align: right">4.36 M</td>
    <td style="white-space: nowrap; text-align: right">229.57 ns</td>
    <td style="white-space: nowrap; text-align: right">±573.04%</td>
    <td style="white-space: nowrap; text-align: right">180 ns</td>
    <td style="white-space: nowrap; text-align: right">347.32 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">4.05 M</td>
    <td style="white-space: nowrap; text-align: right">246.79 ns</td>
    <td style="white-space: nowrap; text-align: right">±526.84%</td>
    <td style="white-space: nowrap; text-align: right">191 ns</td>
    <td style="white-space: nowrap; text-align: right">354.13 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">1.20 M</td>
    <td style="white-space: nowrap; text-align: right">830.19 ns</td>
    <td style="white-space: nowrap; text-align: right">±276.61%</td>
    <td style="white-space: nowrap; text-align: right">672 ns</td>
    <td style="white-space: nowrap; text-align: right">1683.51 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Enum.fetch/2 (list)</td>
    <td style="white-space: nowrap; text-align: right">1.14 M</td>
    <td style="white-space: nowrap; text-align: right">878.73 ns</td>
    <td style="white-space: nowrap; text-align: right">±152.99%</td>
    <td style="white-space: nowrap; text-align: right">799 ns</td>
    <td style="white-space: nowrap; text-align: right">1723.53 ns</td>
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
    <td style="white-space: nowrap;text-align: right">4.36 M</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">4.05 M</td>
    <td style="white-space: nowrap; text-align: right">1.08x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">1.20 M</td>
    <td style="white-space: nowrap; text-align: right">3.62x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Enum.fetch/2 (list)</td>
    <td style="white-space: nowrap; text-align: right">1.14 M</td>
    <td style="white-space: nowrap; text-align: right">3.83x</td>
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
    <td style="white-space: nowrap; text-align: right">4.07 M</td>
    <td style="white-space: nowrap; text-align: right">245.98 ns</td>
    <td style="white-space: nowrap; text-align: right">±588.44%</td>
    <td style="white-space: nowrap; text-align: right">186 ns</td>
    <td style="white-space: nowrap; text-align: right">417.11 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">3.88 M</td>
    <td style="white-space: nowrap; text-align: right">257.54 ns</td>
    <td style="white-space: nowrap; text-align: right">±573.94%</td>
    <td style="white-space: nowrap; text-align: right">189 ns</td>
    <td style="white-space: nowrap; text-align: right">340.61 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">1.25 M</td>
    <td style="white-space: nowrap; text-align: right">797.48 ns</td>
    <td style="white-space: nowrap; text-align: right">±273.90%</td>
    <td style="white-space: nowrap; text-align: right">668 ns</td>
    <td style="white-space: nowrap; text-align: right">1418.80 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Enum.fetch/2 (list)</td>
    <td style="white-space: nowrap; text-align: right">0.60 M</td>
    <td style="white-space: nowrap; text-align: right">1672.07 ns</td>
    <td style="white-space: nowrap; text-align: right">±112.55%</td>
    <td style="white-space: nowrap; text-align: right">1522 ns</td>
    <td style="white-space: nowrap; text-align: right">3343.12 ns</td>
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
    <td style="white-space: nowrap;text-align: right">4.07 M</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">3.88 M</td>
    <td style="white-space: nowrap; text-align: right">1.05x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">1.25 M</td>
    <td style="white-space: nowrap; text-align: right">3.24x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Enum.fetch/2 (list)</td>
    <td style="white-space: nowrap; text-align: right">0.60 M</td>
    <td style="white-space: nowrap; text-align: right">6.8x</td>
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
    <td style="white-space: nowrap; text-align: right">4.18 M</td>
    <td style="white-space: nowrap; text-align: right">239.14 ns</td>
    <td style="white-space: nowrap; text-align: right">±470.79%</td>
    <td style="white-space: nowrap; text-align: right">197 ns</td>
    <td style="white-space: nowrap; text-align: right">315.98 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">3.82 M</td>
    <td style="white-space: nowrap; text-align: right">261.62 ns</td>
    <td style="white-space: nowrap; text-align: right">±475.57%</td>
    <td style="white-space: nowrap; text-align: right">198 ns</td>
    <td style="white-space: nowrap; text-align: right">348.98 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">1.23 M</td>
    <td style="white-space: nowrap; text-align: right">814.00 ns</td>
    <td style="white-space: nowrap; text-align: right">±272.32%</td>
    <td style="white-space: nowrap; text-align: right">679 ns</td>
    <td style="white-space: nowrap; text-align: right">1215 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Enum.fetch/2 (list)</td>
    <td style="white-space: nowrap; text-align: right">0.31 M</td>
    <td style="white-space: nowrap; text-align: right">3242.77 ns</td>
    <td style="white-space: nowrap; text-align: right">±86.57%</td>
    <td style="white-space: nowrap; text-align: right">3044 ns</td>
    <td style="white-space: nowrap; text-align: right">18925.02 ns</td>
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
    <td style="white-space: nowrap;text-align: right">4.18 M</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">3.82 M</td>
    <td style="white-space: nowrap; text-align: right">1.09x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">1.23 M</td>
    <td style="white-space: nowrap; text-align: right">3.4x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Enum.fetch/2 (list)</td>
    <td style="white-space: nowrap; text-align: right">0.31 M</td>
    <td style="white-space: nowrap; text-align: right">13.56x</td>
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
    <td style="white-space: nowrap; text-align: right">3.71 M</td>
    <td style="white-space: nowrap; text-align: right">269.34 ns</td>
    <td style="white-space: nowrap; text-align: right">±619.91%</td>
    <td style="white-space: nowrap; text-align: right">205 ns</td>
    <td style="white-space: nowrap; text-align: right">363.15 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">3.56 M</td>
    <td style="white-space: nowrap; text-align: right">280.70 ns</td>
    <td style="white-space: nowrap; text-align: right">±643.96%</td>
    <td style="white-space: nowrap; text-align: right">206 ns</td>
    <td style="white-space: nowrap; text-align: right">484.56 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">1.17 M</td>
    <td style="white-space: nowrap; text-align: right">856.43 ns</td>
    <td style="white-space: nowrap; text-align: right">±347.01%</td>
    <td style="white-space: nowrap; text-align: right">667 ns</td>
    <td style="white-space: nowrap; text-align: right">1514.92 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Enum.fetch/2 (list)</td>
    <td style="white-space: nowrap; text-align: right">0.162 M</td>
    <td style="white-space: nowrap; text-align: right">6166.10 ns</td>
    <td style="white-space: nowrap; text-align: right">±72.92%</td>
    <td style="white-space: nowrap; text-align: right">5821.50 ns</td>
    <td style="white-space: nowrap; text-align: right">25194.94 ns</td>
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
    <td style="white-space: nowrap;text-align: right">3.71 M</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">3.56 M</td>
    <td style="white-space: nowrap; text-align: right">1.04x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">1.17 M</td>
    <td style="white-space: nowrap; text-align: right">3.18x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Enum.fetch/2 (list)</td>
    <td style="white-space: nowrap; text-align: right">0.162 M</td>
    <td style="white-space: nowrap; text-align: right">22.89x</td>
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
    <td style="white-space: nowrap; text-align: right">4.01 M</td>
    <td style="white-space: nowrap; text-align: right">249.39 ns</td>
    <td style="white-space: nowrap; text-align: right">±95.97%</td>
    <td style="white-space: nowrap; text-align: right">240 ns</td>
    <td style="white-space: nowrap; text-align: right">411.93 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">3.90 M</td>
    <td style="white-space: nowrap; text-align: right">256.56 ns</td>
    <td style="white-space: nowrap; text-align: right">±225.60%</td>
    <td style="white-space: nowrap; text-align: right">229 ns</td>
    <td style="white-space: nowrap; text-align: right">361 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">1.14 M</td>
    <td style="white-space: nowrap; text-align: right">879.76 ns</td>
    <td style="white-space: nowrap; text-align: right">±278.74%</td>
    <td style="white-space: nowrap; text-align: right">715 ns</td>
    <td style="white-space: nowrap; text-align: right">1904.60 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Enum.fetch/2 (list)</td>
    <td style="white-space: nowrap; text-align: right">0.0802 M</td>
    <td style="white-space: nowrap; text-align: right">12469.74 ns</td>
    <td style="white-space: nowrap; text-align: right">±68.81%</td>
    <td style="white-space: nowrap; text-align: right">12041 ns</td>
    <td style="white-space: nowrap; text-align: right">47541.12 ns</td>
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
    <td style="white-space: nowrap;text-align: right">4.01 M</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">3.90 M</td>
    <td style="white-space: nowrap; text-align: right">1.03x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">1.14 M</td>
    <td style="white-space: nowrap; text-align: right">3.53x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Enum.fetch/2 (list)</td>
    <td style="white-space: nowrap; text-align: right">0.0802 M</td>
    <td style="white-space: nowrap; text-align: right">50.0x</td>
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
    <td style="white-space: nowrap; text-align: right">3.59 M</td>
    <td style="white-space: nowrap; text-align: right">278.36 ns</td>
    <td style="white-space: nowrap; text-align: right">±259.47%</td>
    <td style="white-space: nowrap; text-align: right">239 ns</td>
    <td style="white-space: nowrap; text-align: right">540.03 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">3.55 M</td>
    <td style="white-space: nowrap; text-align: right">281.37 ns</td>
    <td style="white-space: nowrap; text-align: right">±122.78%</td>
    <td style="white-space: nowrap; text-align: right">255 ns</td>
    <td style="white-space: nowrap; text-align: right">578.72 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">1.08 M</td>
    <td style="white-space: nowrap; text-align: right">922.96 ns</td>
    <td style="white-space: nowrap; text-align: right">±253.71%</td>
    <td style="white-space: nowrap; text-align: right">761 ns</td>
    <td style="white-space: nowrap; text-align: right">2199.17 ns</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Enum.fetch/2 (list)</td>
    <td style="white-space: nowrap; text-align: right">0.0424 M</td>
    <td style="white-space: nowrap; text-align: right">23569.84 ns</td>
    <td style="white-space: nowrap; text-align: right">±64.76%</td>
    <td style="white-space: nowrap; text-align: right">22695 ns</td>
    <td style="white-space: nowrap; text-align: right">74334.40 ns</td>
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
    <td style="white-space: nowrap;text-align: right">3.59 M</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">3.55 M</td>
    <td style="white-space: nowrap; text-align: right">1.01x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (RRBVector)</td>
    <td style="white-space: nowrap; text-align: right">1.08 M</td>
    <td style="white-space: nowrap; text-align: right">3.32x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Enum.fetch/2 (list)</td>
    <td style="white-space: nowrap; text-align: right">0.0424 M</td>
    <td style="white-space: nowrap; text-align: right">84.67x</td>
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
    <td style="white-space: nowrap; text-align: right">1020.07 K</td>
    <td style="white-space: nowrap; text-align: right">0.98 μs</td>
    <td style="white-space: nowrap; text-align: right">±280.94%</td>
    <td style="white-space: nowrap; text-align: right">0.79 μs</td>
    <td style="white-space: nowrap; text-align: right">3.09 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">650.27 K</td>
    <td style="white-space: nowrap; text-align: right">1.54 μs</td>
    <td style="white-space: nowrap; text-align: right">±1676.07%</td>
    <td style="white-space: nowrap; text-align: right">0.25 μs</td>
    <td style="white-space: nowrap; text-align: right">0.91 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">642.68 K</td>
    <td style="white-space: nowrap; text-align: right">1.56 μs</td>
    <td style="white-space: nowrap; text-align: right">±1639.17%</td>
    <td style="white-space: nowrap; text-align: right">0.26 μs</td>
    <td style="white-space: nowrap; text-align: right">1.16 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Enum.fetch/2 (list)</td>
    <td style="white-space: nowrap; text-align: right">20.79 K</td>
    <td style="white-space: nowrap; text-align: right">48.10 μs</td>
    <td style="white-space: nowrap; text-align: right">±64.26%</td>
    <td style="white-space: nowrap; text-align: right">47.05 μs</td>
    <td style="white-space: nowrap; text-align: right">139.59 μs</td>
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
    <td style="white-space: nowrap;text-align: right">1020.07 K</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">650.27 K</td>
    <td style="white-space: nowrap; text-align: right">1.57x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">642.68 K</td>
    <td style="white-space: nowrap; text-align: right">1.59x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Enum.fetch/2 (list)</td>
    <td style="white-space: nowrap; text-align: right">20.79 K</td>
    <td style="white-space: nowrap; text-align: right">49.06x</td>
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
    <td style="white-space: nowrap; text-align: right">1033.39 K</td>
    <td style="white-space: nowrap; text-align: right">0.97 μs</td>
    <td style="white-space: nowrap; text-align: right">±255.43%</td>
    <td style="white-space: nowrap; text-align: right">0.79 μs</td>
    <td style="white-space: nowrap; text-align: right">2.40 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">484.29 K</td>
    <td style="white-space: nowrap; text-align: right">2.06 μs</td>
    <td style="white-space: nowrap; text-align: right">±80.51%</td>
    <td style="white-space: nowrap; text-align: right">1.74 μs</td>
    <td style="white-space: nowrap; text-align: right">14.97 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">481.37 K</td>
    <td style="white-space: nowrap; text-align: right">2.08 μs</td>
    <td style="white-space: nowrap; text-align: right">±124.97%</td>
    <td style="white-space: nowrap; text-align: right">1.57 μs</td>
    <td style="white-space: nowrap; text-align: right">18.57 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Enum.fetch/2 (list)</td>
    <td style="white-space: nowrap; text-align: right">10.35 K</td>
    <td style="white-space: nowrap; text-align: right">96.57 μs</td>
    <td style="white-space: nowrap; text-align: right">±62.35%</td>
    <td style="white-space: nowrap; text-align: right">93.14 μs</td>
    <td style="white-space: nowrap; text-align: right">249.97 μs</td>
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
    <td style="white-space: nowrap;text-align: right">1033.39 K</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">484.29 K</td>
    <td style="white-space: nowrap; text-align: right">2.13x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">481.37 K</td>
    <td style="white-space: nowrap; text-align: right">2.15x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Enum.fetch/2 (list)</td>
    <td style="white-space: nowrap; text-align: right">10.35 K</td>
    <td style="white-space: nowrap; text-align: right">99.8x</td>
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
    <td style="white-space: nowrap; text-align: right">1010.13 K</td>
    <td style="white-space: nowrap; text-align: right">0.99 μs</td>
    <td style="white-space: nowrap; text-align: right">±241.45%</td>
    <td style="white-space: nowrap; text-align: right">0.79 μs</td>
    <td style="white-space: nowrap; text-align: right">3.51 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">367.47 K</td>
    <td style="white-space: nowrap; text-align: right">2.72 μs</td>
    <td style="white-space: nowrap; text-align: right">±59.25%</td>
    <td style="white-space: nowrap; text-align: right">2.39 μs</td>
    <td style="white-space: nowrap; text-align: right">12.66 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">358.81 K</td>
    <td style="white-space: nowrap; text-align: right">2.79 μs</td>
    <td style="white-space: nowrap; text-align: right">±78.53%</td>
    <td style="white-space: nowrap; text-align: right">2.47 μs</td>
    <td style="white-space: nowrap; text-align: right">19.59 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Enum.fetch/2 (list)</td>
    <td style="white-space: nowrap; text-align: right">4.37 K</td>
    <td style="white-space: nowrap; text-align: right">228.77 μs</td>
    <td style="white-space: nowrap; text-align: right">±62.52%</td>
    <td style="white-space: nowrap; text-align: right">216.59 μs</td>
    <td style="white-space: nowrap; text-align: right">547.85 μs</td>
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
    <td style="white-space: nowrap;text-align: right">1010.13 K</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">367.47 K</td>
    <td style="white-space: nowrap; text-align: right">2.75x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">358.81 K</td>
    <td style="white-space: nowrap; text-align: right">2.82x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Enum.fetch/2 (list)</td>
    <td style="white-space: nowrap; text-align: right">4.37 K</td>
    <td style="white-space: nowrap; text-align: right">231.09x</td>
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
    <td style="white-space: nowrap; text-align: right">889.95 K</td>
    <td style="white-space: nowrap; text-align: right">1.12 μs</td>
    <td style="white-space: nowrap; text-align: right">±229.78%</td>
    <td style="white-space: nowrap; text-align: right">0.84 μs</td>
    <td style="white-space: nowrap; text-align: right">4.93 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">403.38 K</td>
    <td style="white-space: nowrap; text-align: right">2.48 μs</td>
    <td style="white-space: nowrap; text-align: right">±14.60%</td>
    <td style="white-space: nowrap; text-align: right">2.42 μs</td>
    <td style="white-space: nowrap; text-align: right">3.59 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">400.78 K</td>
    <td style="white-space: nowrap; text-align: right">2.50 μs</td>
    <td style="white-space: nowrap; text-align: right">±15.39%</td>
    <td style="white-space: nowrap; text-align: right">2.41 μs</td>
    <td style="white-space: nowrap; text-align: right">4.16 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Enum.fetch/2 (list)</td>
    <td style="white-space: nowrap; text-align: right">1.78 K</td>
    <td style="white-space: nowrap; text-align: right">560.27 μs</td>
    <td style="white-space: nowrap; text-align: right">±50.88%</td>
    <td style="white-space: nowrap; text-align: right">581.41 μs</td>
    <td style="white-space: nowrap; text-align: right">1079.69 μs</td>
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
    <td style="white-space: nowrap;text-align: right">889.95 K</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">403.38 K</td>
    <td style="white-space: nowrap; text-align: right">2.21x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">400.78 K</td>
    <td style="white-space: nowrap; text-align: right">2.22x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Enum.fetch/2 (list)</td>
    <td style="white-space: nowrap; text-align: right">1.78 K</td>
    <td style="white-space: nowrap; text-align: right">498.61x</td>
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
    <td style="white-space: nowrap; text-align: right">937.40 K</td>
    <td style="white-space: nowrap; text-align: right">1.07 μs</td>
    <td style="white-space: nowrap; text-align: right">±225.83%</td>
    <td style="white-space: nowrap; text-align: right">0.86 μs</td>
    <td style="white-space: nowrap; text-align: right">3.44 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">363.72 K</td>
    <td style="white-space: nowrap; text-align: right">2.75 μs</td>
    <td style="white-space: nowrap; text-align: right">±27.40%</td>
    <td style="white-space: nowrap; text-align: right">2.48 μs</td>
    <td style="white-space: nowrap; text-align: right">5.37 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">272.47 K</td>
    <td style="white-space: nowrap; text-align: right">3.67 μs</td>
    <td style="white-space: nowrap; text-align: right">±106.83%</td>
    <td style="white-space: nowrap; text-align: right">2.58 μs</td>
    <td style="white-space: nowrap; text-align: right">25.43 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Enum.fetch/2 (list)</td>
    <td style="white-space: nowrap; text-align: right">0.99 K</td>
    <td style="white-space: nowrap; text-align: right">1010.34 μs</td>
    <td style="white-space: nowrap; text-align: right">±66.38%</td>
    <td style="white-space: nowrap; text-align: right">946.04 μs</td>
    <td style="white-space: nowrap; text-align: right">2858.88 μs</td>
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
    <td style="white-space: nowrap;text-align: right">937.40 K</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">363.72 K</td>
    <td style="white-space: nowrap; text-align: right">2.58x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">272.47 K</td>
    <td style="white-space: nowrap; text-align: right">3.44x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Enum.fetch/2 (list)</td>
    <td style="white-space: nowrap; text-align: right">0.99 K</td>
    <td style="white-space: nowrap; text-align: right">947.1x</td>
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
    <td style="white-space: nowrap; text-align: right">830.11 K</td>
    <td style="white-space: nowrap; text-align: right">1.20 μs</td>
    <td style="white-space: nowrap; text-align: right">±220.05%</td>
    <td style="white-space: nowrap; text-align: right">0.89 μs</td>
    <td style="white-space: nowrap; text-align: right">4.77 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">293.80 K</td>
    <td style="white-space: nowrap; text-align: right">3.40 μs</td>
    <td style="white-space: nowrap; text-align: right">±31.96%</td>
    <td style="white-space: nowrap; text-align: right">2.91 μs</td>
    <td style="white-space: nowrap; text-align: right">6.99 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">231.88 K</td>
    <td style="white-space: nowrap; text-align: right">4.31 μs</td>
    <td style="white-space: nowrap; text-align: right">±76.99%</td>
    <td style="white-space: nowrap; text-align: right">3.48 μs</td>
    <td style="white-space: nowrap; text-align: right">17.14 μs</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Enum.fetch/2 (list)</td>
    <td style="white-space: nowrap; text-align: right">0.45 K</td>
    <td style="white-space: nowrap; text-align: right">2208.76 μs</td>
    <td style="white-space: nowrap; text-align: right">±59.59%</td>
    <td style="white-space: nowrap; text-align: right">2071.48 μs</td>
    <td style="white-space: nowrap; text-align: right">4560.29 μs</td>
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
    <td style="white-space: nowrap;text-align: right">830.11 K</td>
    <td>&nbsp;</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (ErlangArray)</td>
    <td style="white-space: nowrap; text-align: right">293.80 K</td>
    <td style="white-space: nowrap; text-align: right">2.83x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Arrays.get/2 (MapArray)</td>
    <td style="white-space: nowrap; text-align: right">231.88 K</td>
    <td style="white-space: nowrap; text-align: right">3.58x</td>
  </tr>

  <tr>
    <td style="white-space: nowrap">Enum.fetch/2 (list)</td>
    <td style="white-space: nowrap; text-align: right">0.45 K</td>
    <td style="white-space: nowrap; text-align: right">1833.52x</td>
  </tr>

</table>



<hr/>

