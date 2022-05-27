defmodule ArraysRRBVector.MixProject do
  use Mix.Project
  @source_url "https://github.com/Qqwy/elixir-arrays_rrb_vector"

  def project do
    [
      app: :arrays_rrb_vector,
      version: "0.2.0",
      elixir: "~> 1.11",
      start_permanent: Mix.env() == :prod,
      deps: deps(),
      name: "ArraysRRBVector",
      description: description(),
      source_url: @source_url,
      package: package()
    ]
  end

  # Run "mix help compile.app" to learn about applications.
  def application do
    [
      mod: {ArraysRRBVector.Application, []},
      extra_applications: [:logger]
    ]
  end

  # Run "mix help deps" to learn about dependencies.
  defp deps do
    [
      {:rustler, "~> 0.25"},
      {:rustler_elixir_fun, "~> 0.3.0"},
      {:arrays, "~> 2.0"},

      {:ex_doc, "~> 0.23", only: :dev, runtime: false},
      {:dialyxir, "~> 1.0", only: [:dev], runtime: false},
      {:credo, "~> 1.5", only: [:dev, :test], runtime: false},
      {:excoveralls, "~> 0.13", only: [:test]},

      {:benchee, [git: "https://github.com/bencheeorg/benchee", only: :dev, override: true]},
      {:benchee_csv, "~> 1.0", only: :dev},
      {:benchee_markdown, "~> 0.2", only: :dev},
      {:benchee_html, "~> 1.0", only: :dev }
    ]
  end

  defp description do
     """
     An `Arrays` implementation based on a set of NIFs (Natively Implemented Functions) written in Rust.
     The internal representation of the array is known as a 'Relaxed Radix Balanced Vector', provided by the Rust `im` library.
     Performance is unfortunately overshadowed by NIF-calling overhead.
     """
  end

  defp package() do
    [
      name: :arrays_rrb_vector,
      files: ["lib", "native/arrays_rrb_vector/src", "native/arrays_rrb_vector/Cargo.toml", "mix.exs", "README*"],
      maintainers: ["Qqwy/Wiebe-Marten Wijnja"],
      licenses: ["Apache 2.0"],
      links: %{"GitHub" => @source_url}
    ]
  end

end
