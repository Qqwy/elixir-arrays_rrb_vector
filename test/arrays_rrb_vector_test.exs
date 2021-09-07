defmodule ArraysRRBVectorTest do
  use ExUnit.Case
  doctest ArraysRRBVector

  test "NIF is able to handle many types" do
    list = [self(), 10, 10.5, "hello", [], 'world', {1, 2, 3}]
    arr = Enum.reduce(list, ArraysRRBVector.empty, fn elem, acc ->
      ArraysRRBVector.append(acc, elem)
    end)
    assert list == ArraysRRBVector.to_list(arr)
  end
end
