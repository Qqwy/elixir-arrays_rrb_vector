defmodule ArraysRRBVector.Application do
  use Application

  def start(_type, _args) do
    children = [
      {RustlerElixirFun.FunExecutionServer, [name: ArraysRRBVector.Mapper]}
    ]

    Supervisor.start_link(children, strategy: :one_for_one)
  end
end
