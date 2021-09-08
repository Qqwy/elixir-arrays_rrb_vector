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

  iex> ArraysRRBVector.new(1..10) |> ArraysRRBVector.slice(2, 3)
  #ArraysRRBVector<[3, 4, 5]>
  """
  def slice(%__MODULE__{handle: handle}, lower, amount) when lower >= 0 do
    if lower + amount < size_impl(handle) do
               new_handle = slice_impl(handle, lower, lower + amount)
               %__MODULE__{handle: new_handle}
               else
                 raise ArgumentError
    end
  end

  @doc false
  def slice_impl(_handle, _lower, _higher), do: nif_error()

  @doc """
  Maps a value over a vector, returning another vector.

  Note that the current implementation is relatively slow,
  as we turn the whole vector in a list,
  map over this list, and then turn the list back into a vector.
  A more effient implementation is probably possible, but a bit tricky to write.
  (PR's welcome! 😉)
  """
  def map(vector, fun) do
    fun
    |> :lists.map(to_list(vector))
    |> new()
  end


  @behaviour Access

  @impl Access
  def fetch(%__MODULE__{handle: handle}, index) when index >= 0 do
    if index >= size_impl(handle) do
      :error
    else
      {:ok, get_impl(handle, index)}
    end
  end

  def fetch(%__MODULE__{handle: handle}, index) when index < 0 do
    size = size_impl(handle)

    if index < (-size) do
      :error
    else
      {:ok, get_impl(handle, index + size)}
    end
  end

  @undefined_pop_message """
  There is no efficient implementation possible to remove an element from a random location in an array, so `Access.pop/2` (and returning `:pop` from `Access.get_and_update/3` ) are not supported by #{inspect(__MODULE__)}. If you want to remove the last element, use `Arrays.extract/1`.
  """ |> String.trim

  @impl Access
  def get_and_update(%__MODULE__{handle: handle}, index, function) when index >= 0 do
    if index >= size_impl(handle) do
      raise ArgumentError
    else
      value = get_impl(handle, index)
      case function.(value) do
        {get, new_value} ->
          new_handle = replace_impl(handle, index, new_value)
          {get, %__MODULE__{handle: new_handle}}
        :pop ->
          raise ArgumentError, @undefined_pop_message
      end
    end
  end

  @impl Access
  def get_and_update(array = %__MODULE__{handle: handle}, index, function) when index < 0 do
    size = size_impl(handle)
    if index < size do
      raise ArgumentError
    else
      get_and_update(array, index + size, function)
    end
  end

  @impl Access
  def pop(%__MODULE__{}, _index) do
    raise ArgumentError, @undefined_pop_message
  end


  defimpl Arrays.Protocol do
    defdelegate size(vector), to: @for
    defdelegate map(vector, fun), to: @for
    defdelegate reduce(vector, acc, fun), to: @for
    defdelegate reduce_right(vector, acc, fun), to: @for
    defdelegate get(vector, index), to: @for
    defdelegate replace(vector, index, item), to: @for
    defdelegate append(vector, item), to: @for
    defdelegate extract(vector), to: @for
    defdelegate resize(vector, size, default), to: @for
    defdelegate to_list(vector), to: @for
    defdelegate slice(vector, start_index, amount), to: @for
    def empty(_), do:  @for.empty()
  end

  defimpl Enumerable do
    def member?(_array, _item), do: {:error, @for}

    def count(array) do
      {:ok, @for.size(array)}
    end

    def reduce(array, acc, fun) do
      size = @for.size(array)
      do_reduce(array, acc, fun, 0, size)
    end

    defp do_reduce(_, {:halt, acc}, _, _, _) do
      {:halted, acc}
    end

    defp do_reduce(array, {:suspend, acc}, fun, index, size) do
      {:suspended, acc, &do_reduce(array, &1, fun, index, size)}
    end

    defp do_reduce(_array, {:cont, acc}, _fun, index, index) do
      {:done, acc}
    end

    defp do_reduce(array, {:cont, acc}, fun, index, size) do
      elem = @for.get(array, index)
      do_reduce(array, fun.(elem, acc), fun, index + 1, size)
    end

    def slice(array) do
      size = @for.size(array)
      builder = fn a, b ->
        array
        |> @for.slice(a, b)
        |> @for.to_list()
      end
      {:ok, size, builder}
    end
  end


  defimpl Extractable do
    def extract(array) do
      Arrays.Protocol.extract(array)
    end
  end

  defimpl Insertable do
    def insert(array, item) do
      {:ok, @for.append(array, item)}
    end
  end
end
