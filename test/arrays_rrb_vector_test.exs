defmodule ArraysRRBVectorTest do
  use ExUnit.Case
  doctest ArraysRRBVector

  test "NIF is able to handle many types" do
    list = [self(), 10, 10.5, "hello", [], 'world', {1, 2, 3}, &Kernel.+/2, make_ref(), <<1>>]
    arr = Enum.reduce(list, ArraysRRBVector.empty, fn elem, acc ->
      ArraysRRBVector.append(acc, elem)
    end)
    assert list == ArraysRRBVector.to_list(arr)
  end

  describe "Collectable" do
    test "works" do
      arr = (1..100)
      |> Enum.into(ArraysRRBVector.empty)

      assert Enum.to_list(1..100) == ArraysRRBVector.to_list(arr)
    end
  end
end
