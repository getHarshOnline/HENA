# Working on HENA

HENA investigates whether many small reasoning units can learn sparse communication, bounded differentiable memory and useful hierarchy from local rules. It is the swarm layer of the Causality research family: the intended first swarm would compose stock open-weight models through MUON's unit interface, with MONAD supplying formal types and Reckon measuring records. Today the kernel only re-exports dependencies and implements no scheduler, routing policy or swarm store: routing, shared memory and scheduling are the work in front of it, and emergent hierarchy is the hypothesis they test.

Start with the [README](README.md), [development overview](docs/index.md) and [build guide](docs/building.md). The overview explains the proposed responsibilities and comparisons; the [kernel manifest](kernel/Cargo.toml) and [crate root](kernel/src/lib.rs) show what actually compiles.

## Work from the current source

`kernel/` depends on MONAD, MUON and Reckon. Reckon's repository, package and Rust import are `reckon`, `reckon-kernel` and `reckon_kernel`. `kernel/Cargo.toml` pins each dependency to one commit of its GitHub repository, so a fresh clone builds alone.

`paper/` holds the draft and imports shared MONAD and MUON definitions. `protos/hena/` contains headers without messages and exposes no swarm API. A proposed allocator, checkpoint or learning mechanism in the manual is not an available interface.

HENA would own its store, retention, pruning and recovery. Reckon's record protocol would support evaluation without transferring ownership of that store. Compare collective behaviour against a single unit, a flat collective, a central coordinator and an imposed hierarchy at matched compute, communication, memory and interaction cost. Improved units and improved coordination are separate explanations; a successful build supplies neither result.

## Check the dependency wiring and paper

Use Rust and Cargo, with network access for the first fetch of the pinned dependencies. Reckon's default Rust build keeps its Python binding disabled and supplies its Protocol Buffers compiler through Cargo; `PROTOC` can select a compiler explicitly. From this repository's root:

```sh
cargo check --manifest-path kernel/Cargo.toml
(cd paper && make all)
```

The Rust command checks that the re-exports compile together; HENA has no swarm implementation or behavioural test suite. The paper command needs the documented Make, Bash and TeX tools and produces `paper/HENA.pdf`; the Makefile selects `HENA.tex`, the only root `.tex` file, so no `MASTER=` is needed. Inspect that PDF and the build warnings when changing the paper; typesetting the proposal establishes no performance or emergence claim.

<!-- BEGIN shared repository rules -->

- Review a source file above 1,000 lines and split it by concern before 1,500.
- Keep Rust tests under `tests/`; keep Go's `_test.go` files beside their package.
- Finish a goal with the codebase the same size or smaller unless a requested feature requires growth. Preserve the behaviour, tests, cases and examples.
- Every engine must build from a fresh clone on Linux and macOS.
- Keep generated API reference and build output out of Git; change their source or generator.
- Write comments and API documentation for the code's reader. A crate or module comment first says what it is for in the system, what it builds on and what builds on it, then its purpose, inputs, results and useful edge cases, with the purpose written problem-first in the present tense: the problem it solves and what solves it today. One plain line after that opening says what is not built yet, as the problem now in front of it, and a negative appears only where a reader would expect the opposite: a limit stated as a property of what exists stays ("it runs on loopback with no caller authentication"), while a negative measured against the finished design is a hedge. Keep them self-contained, with links inside the repository.
- Push each repository as one commit whose whole message is `init`, with Harsh as author and committer under the account the folder uses and no `Co-Authored-By` or `Claude-Session` trailers; later commits stay short and plain.
- Describe the present design and research. The repository neither links nor ships lineage: earlier papers, predecessor versions, history tags and the commits behind them.
- Public repository files, including generated text, link within the repository or to public addresses their readers can open.
- A README is the repository's front door: it carries the product account for the technical reader who arrives there, opening with the product's plain name and the problem it solves, in the words its published pages use.
- Name every operation, command, tool, type, field and variable so a reader without its author's context can tell what it does, what it enables and when to use it; docstrings say the same. Keep full-field record schemas for the engine's consistency, named in plain words like everything else, and author each surface's schemas separately, one per intent, in its caller's terms: SDKs and APIs, the command line, the language server and the agent protocols (MCP, A2A, ACP) are different surfaces with different callers, and the agent protocols compose intents over the operations the others expose.
- Write each Markdown paragraph on one source line. Run the repository's formatter with `proseWrap: never` on changed Markdown.

<!-- END shared repository rules -->

Before handing back:

- Does the entry or change preserve HENA's research purpose while describing only the capabilities that exist?
- Are unit behaviour, swarm coordination and ownership of the swarm's store kept distinct?
- What did the dependency or paper check establish, and what remains a research target or an unchecked dependency?
