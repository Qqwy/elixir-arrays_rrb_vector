defmodule ArraysRRBVector do
  use Rustler, otp_app: :arrays_rrb_vector, crate: :arrays_rrb_vector
  @moduledoc """
  Documentation for `ArraysRRBVector`.
  """


  @type t() :: %__MODULE__{}
  defstruct [:contents]

  @doc """
  Returns an empty RRBVector.

      iex> ArraysRRBVector.empty()
      #ArraysRRBVector<[]>
  """
  def empty() do
    %__MODULE__{contents: empty_impl()}
  end
  def empty_impl(), do: :erlang.nif_error(:nif_not_loaded)

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
  def size_impl(_vector), do: :erlang.nif_error(:nif_not_loaded)

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
  def append(%__MODULE__{contents: contents}, item) do
    case append_impl(contents, item) do
      {:ok, new_contents} ->
        %__MODULE__{contents: new_contents}
      {:error, :unsupported_type} ->
        raise ArgumentError
    end
  end

  def append_impl(_vector, _item), do: :erlang.nif_error(:nif_not_loaded)

  def to_list(%__MODULE__{contents: contents}) do
    to_list_impl(contents)
  end

  def to_list_impl(_vector), do: :erlang.nif_error(:nif_not_loaded)



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
end
