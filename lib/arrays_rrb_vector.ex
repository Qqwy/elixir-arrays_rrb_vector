defmodule ArraysRRBVector do
  use Rustler, otp_app: :arrays_rrb_vector, crate: :arrays_rrb_vector


  @nif_error (quote do :erlang.nif_error(:nif_not_loaded) end)

  @moduledoc """
  Documentation for `ArraysRRBVector`.
  """


  @type t() :: %__MODULE__{}
  # def __struct__() do
  #   %{__struct__: __MODULE__, contents: empty_impl()}
  # end
  defstruct [:contents]
  # def __struct__(kv) do
  #   Enum.reduce(kv, __struct__(), fn {k, v}, acc -> :maps.update(k, v, acc) end)
  # end

  @doc """
  Returns an empty RRBVector.

      iex> ArraysRRBVector.empty()
      #ArraysRRBVector<[]>
  """
  def empty() do
    %__MODULE__{contents: empty_impl()}
  end

  @doc false
  def empty_impl(), do: @nif_error

  @doc """
  The number of elements in `vector`.

      iex> ArraysRRBVector.size(ArraysRRBVector.empty())
      0

      iex> ArraysRRBVector.size(ArraysRRBVector.append(ArraysRRBVector.empty(), 42))
      1
  """
  def size(%__MODULE__{contents: contents}) do
    size_impl(contents)
  end

  @doc false
  def size_impl(_vector), do: 0 # @nif_error

  @doc """
  Appends an element to an RRBVector.

  Note that currently not all Elixir/Erlang terms are supported,
  because of limitations of NIFs.

  In the case this function is called with an unsupported kind of term,
  `ArgumentError` is raised.

  Not currently supported are:

   - References
   - PIDs
   - Ports
   - Functions
   - Integers that are larger than would fit in a signed 64-bit number.
  """
  def append(vector, item)
  def append(%__MODULE__{contents: contents}, item) do
    case append_impl(contents, item) do
      {:ok, new_contents} ->
        %__MODULE__{contents: new_contents}
      {:error, :unsupported_type} ->
        raise ArgumentError
    end
  end

  def append_impl(_vector, _item), do: @nif_error


  def extract(vector)
  def extract(%__MODULE__{contents: contents}) do
    case extract_impl(contents) do
      {:ok, {val, new_contents}} ->
        {:ok, {val, %__MODULE__{contents: new_contents}}}
      {:error, :empty} ->
        {:error, :empty}
    end
  end

  def extract_impl(_vector), do: @nif_error

  def to_list(vector)
  def to_list(%__MODULE__{contents: contents}) do
    to_list_impl(contents)
  end

  @doc false
  def to_list_impl(_vector), do: @nif_error

  def reduce(vector, acc, fun)
  def reduce(%__MODULE__{contents: contents}, acc, fun) do
    do_reduce(to_iterator(contents), acc, fun)
  end

  defp do_reduce(iterator, acc, fun) do
    case iterator_next(iterator) do
      {:ok, val} ->
        do_reduce(iterator, fun.(val, acc), fun)
      {:error, :empty} -> acc
    end
  end

  def reduce_right(vector, acc, fun)
  def reduce_right(%__MODULE__{contents: contents}, acc, fun) do
    do_reduce_right(to_iterator(contents), acc, fun)
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
  def to_iterator(_vector), do: @nif_error
  @doc false
  def iterator_next(_vector_iterator), do: @nif_error

  @doc false
  def to_reverse_iterator(_vector), do: @nif_error
  @doc false
  def reverse_iterator_next(_vector_iterator), do: @nif_error

  defimpl Inspect do
    import Inspect.Algebra

    def inspect(%@for{contents: contents}, opts) do
      list = @for.to_list_impl(contents)
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


  def get(%__MODULE__{contents: contents}, index) when is_integer(index) do
    if index in (0..(size_impl(contents) - 1)) do
      get_impl(contents, index)
    else
      raise ArgumentError
    end
  end

  def replace(%__MODULE__{contents: contents}, index, value) when is_integer(index) do
    if index in (0..(size_impl(contents) - 1)) do
      new_contents = replace_impl(contents, index, value)
      %__MODULE__{contents: new_contents}
    else
      raise ArgumentError
    end
  end

  @doc false
  def get_impl(_vector, _index), do: @nif_error

  @doc false
  def replace_impl(_vector, _index, _value), do: @nif_error
end
