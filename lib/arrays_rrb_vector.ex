defmodule ArraysRRBVector do
  defp nif_error(), do: :erlang.nif_error(:nif_not_loaded)

  use Rustler, otp_app: :arrays_rrb_vector, crate: :arrays_rrb_vector


  @moduledoc """
  A NIF-based immutable array, based on a 'Relaxed Radix Balanced Trie Vector'.

  This Elixir module wraps [the vector exposed by the Rust library `im`](https://docs.rs/im/15.0.0/im/vector/struct.Vector.html).
  This is a persistent vector datastructure that contains the following optimizations:

  - Smart head/tail chunking
  - The ability to do updates in-place when there is only one owner of a particular (part of a) vector.

  Most operations run either in O(1), amortized O(1), or O(log64(n)).


  ### Dealing with NIFs

  Because this library is implemented in Rust code, wrapped by a bunch of 'Natively Implemented Function's,
  using the [Rustler](https://github.com/rusterlium/rustler) library for interop,
  there are some things to keep in mind:

  - This module does not support hot-code reloading. (NIFs can theoretically support it, but Rustler currently [does not](https://github.com/rusterlium/rustler/issues/13))
  - The arrays created in this module support all Elixir/Erlang terms.
    For most terms this is performant.
    Because of limitations of the NIF interface, storing/reading the following terms to/from arrays has a bit more overhead:
      - References
      - Functions
      - Integers which are larger than what fits in a 64-bit signed number.
      - Ports

  """


  @type t() :: %__MODULE__{}
  # def __struct__() do
  #   %{__struct__: __MODULE__, handle: empty_impl()}
  # end
  defstruct [:handle]
  # def __struct__(kv) do
  #   Enum.reduce(kv, __struct__(), fn {k, v}, acc -> :maps.update(k, v, acc) end)
  # end

  @doc """
  Returns an empty RRBVector.

      iex> ArraysRRBVector.empty()
      #ArraysRRBVector<[]>
  """
  def empty() do
    %__MODULE__{handle: empty_impl()}
  end

  @doc false
  def empty_impl(), do: nif_error()

  @doc """
  The number of elements in `vector`.

      iex> ArraysRRBVector.size(ArraysRRBVector.empty())
      0

      iex> ArraysRRBVector.size(ArraysRRBVector.append(ArraysRRBVector.empty(), 42))
      1
  """
  def size(%__MODULE__{handle: handle}) do
    size_impl(handle)
  end

  @doc false
  def size_impl(_vector), do: 0 # nif_error()

  @doc """
  Appends an element to an RRBVector.
  """
  def append(vector, item)
  def append(%__MODULE__{handle: handle}, item) do
    new_handle = append_impl(handle, item)
    %__MODULE__{handle: new_handle}
  end

  def append_impl(_vector, _item), do: nif_error()


  def extract(vector)
  def extract(%__MODULE__{handle: handle}) do
    case extract_impl(handle) do
      {:ok, {val, new_handle}} ->
        {:ok, {val, %__MODULE__{handle: new_handle}}}
      {:error, :empty} ->
        {:error, :empty}
    end
  end

  def extract_impl(_vector), do: nif_error()

  def to_list(vector)
  def to_list(%__MODULE__{handle: handle}) do
    to_list_impl(handle)
  end

  @doc false
  def to_list_impl(_vector), do: nif_error()

  def reduce(vector, acc, fun)
  def reduce(%__MODULE__{handle: handle}, acc, fun) do
    do_reduce(to_iterator(handle), acc, fun)
  end

  defp do_reduce(iterator, acc, fun) do
    case iterator_next(iterator) do
      {:ok, val} ->
        do_reduce(iterator, fun.(val, acc), fun)
      {:error, :empty} -> acc
    end
  end

  def reduce_right(vector, acc, fun)
  def reduce_right(%__MODULE__{handle: handle}, acc, fun) do
    do_reduce_right(to_iterator(handle), acc, fun)
  end

  defp do_reduce_right(iterator, acc, fun) do
    case iterator_next(iterator) do
      {:ok, val} ->
        do_reduce_right(iterator, fun.(acc, val), fun)
      {:error, :empty} -> acc
    end
  end

  # NOTE:
  # Iterating over the iterator is _mutable_!
  # Specifically, `iterator_next` will return a different result.
  # However, this is OK since an iterator is intended to only be used for a single iteration.
  # It should never leave this module.
  @doc false
  def to_iterator(_vector), do: nif_error()
  @doc false
  def iterator_next(_vector_iterator), do: nif_error()

  @doc false
  def to_reverse_iterator(_vector), do: nif_error()
  @doc false
  def reverse_iterator_next(_vector_iterator), do: nif_error()

  defimpl Inspect do
    import Inspect.Algebra

    def inspect(%@for{handle: handle}, opts) do
      list = @for.to_list_impl(handle)
      concat([
        "##{inspect(@for)}<",
        Inspect.List.inspect(list, %{opts | charlists: :as_lists}),
        ">"
      ])
    end
  end

  @doc false
  def collectable_fun(vector, command) do
    case command do
      {:cont, elem} ->
        append(vector, elem)
      :done ->
        vector
      :halt ->
        :ok
    end
  end


  defimpl Collectable do
    def into(vector) do
      {vector, &@for.collectable_fun/2}
    end
  end

  def new(enumerable) do
    Enum.into(enumerable, empty())
  end


  def get(%__MODULE__{handle: handle}, index) when is_integer(index) do
    if index in (0..(size_impl(handle) - 1)) do
      get_impl(handle, index)
    else
      raise ArgumentError
    end
  end

  def replace(%__MODULE__{handle: handle}, index, value) when is_integer(index) do
    if index in (0..(size_impl(handle) - 1)) do
      new_handle = replace_impl(handle, index, value)
      %__MODULE__{handle: new_handle}
    else
      raise ArgumentError
    end
  end

  @doc false
  def get_impl(_vector, _index), do: nif_error()

  @doc false
  def replace_impl(_vector, _index, _value), do: nif_error()


  def resize(%__MODULE__{handle: handle}, size, default \\ nil) do
    new_handle = resize_impl(handle, size, default)
    %__MODULE__{handle: new_handle}
  end
  @doc false
  def resize_impl(_handle, _size, _default), do: nif_error()

  @doc """
  Extracts a contiguous subsequence of elements from the RRBVector,
  and returns it as its own RRBVector.

      iex> ArraysRRBVector.new(1..10) |> ArraysRRBVector.slice(2, 5)
      #ArraysRRBVector<[3, 4, 5]>
  """
  def slice(%__MODULE__{handle: handle}, lower, higher) when lower >= 0 and lower <= higher do
    if higher < size_impl(handle) do
      new_handle = slice_impl(handle, lower, higher)
      %__MODULE__{handle: new_handle}
    else
      raise ArgumentError
    end
  end

  @doc false
  def slice_impl(_handle, _lower, _higher), do: nif_error()
end
