# rust-datastructures-algorithms

The purpose of this Rust crate is a library of data structures and sorting and search algorithms that use said data structures.

In March 2025, I completed a grad-level course on data structures and algorithms at Drexel. In this class, we implemented many classic versions of these in C. I am interested in trying to reproduce this work in Rust, as it is the programming language I am primarily interested in developing my skill and familiarity in. In doing this, I aim to more thoroughly understand not only the data structures and algorithms I will implement, but also more extensively understand Rust and its particular features in a working context. I also want to benchmark the results in computer clock cycles of my implementations in Rust to compare against the performance of my implementations in C.


==============================================================================
mason-lspconfig troubleshooting                          *rustaceanvim.mason*

This plugin supports automatically detecting mason.nvim codelldb installations,
but not rust-analyzer.
The main reason for this choice is that it mason.nvim installations of rust-analyzer
will most likely have been built with a different toolchain than your project,
leading to inconsistencies and possibly subtle bugs.

If you want to use a mason.nvim installation anyway, you can do so by specifying
the `server.cmd` setting (see |rustaceanvim.config| and |RustaceanLspClientOpts|):
>lua
  vim.g.rustaceanvim = {
    server = {
      cmd = function()
	local mason_registry = require('mason-registry')
	if mason_registry.is_installed('rust-analyzer') then
	  -- This may need to be tweaked depending on the operating system.
	  local ra = mason_registry.get_package('rust-analyzer')
	  local ra_filename = ra:get_receipt():get().links.bin['rust-analyzer']
	  return { ('%s/%s'):format(ra:get_install_path(), ra_filename or 'rust-analyzer') }
	else
	  -- global installation
	  return { 'rust-analyzer' }
	end
      end,
    },
  }
<
Note that mason-lspconfig.nvim, when configured to ensure rust-analyzer is installed,
assumes you are using the `nvim-lspconfig.rust_analyzer` client.
Some Neovim distributions will automatically call the client's `setup`
function, resulting in a conflict with this plugin.

General approach to prevent mason-lspconfig from setting up
`lspconfig.rust_analyzer`:

>lua
  require('mason-lspconfig').setup_handlers {
    ['rust_analyzer'] = function() end,
  }
<

Using LazyVim:

>lua
  {
    'neovim/nvim-lspconfig',
    opts = {
      setup = {
        rust_analyzer = function()
          return true 
        end,
      },
    },
  }
<
vim:tw=78:ts=8:noet:ft=help:norl:
