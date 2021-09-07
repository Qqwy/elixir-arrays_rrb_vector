defmodule ArraysRRBVector do
  use Rustler, otp_app: :arrays_rrb_vector, crate: :arrays_rrb_vector
  @moduledoc """
  Documentation for `ArraysRRBVector`.
  """

  @doc """
  Hello world.

  ## Examples

      iex> ArraysRRBVector.add(1, 2)
      3

  """
  def add(left, right)
  def add(_left, _right), do: :erlang.nif_error(:nif_not_loaded)


  defstruct [:contents]

  def empty() do
    %__MODULE__{contents: empty_impl()}
  end
  def empty_impl(), do: :erlang.nif_error(:nif_not_loaded)

  def size(%__MODULE__{contents: contents}) do
    size_impl(contents)
  end
  def size_impl(_vector), do: :erlang.nif_error(:nif_not_loaded)

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
