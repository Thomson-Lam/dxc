return {
  -- Keep LazyVim's LSP/Mason framework, and only add the servers/tools
  -- from nvim-config-2026 that we want to preserve.
  {
    "neovim/nvim-lspconfig",
    opts = {
      servers = {
        clangd = {},
        gopls = {},
        pyright = {},
        rust_analyzer = {},
        ts_ls = {},
        superhtml = {},
        lua_ls = {
          settings = {
            Lua = {
              completion = {
                callSnippet = "Replace",
              },
            },
          },
        },
      },
    },
  },

  -- LSP progress/status UI from nvim-config-2026.
  {
    "j-hui/fidget.nvim",
    event = "LspAttach",
    opts = {},
  },

  -- Arduino workflow integration from nvim-config-2026.
  {
    "glebzlat/arduino-nvim",
    ft = "arduino",
    config = function()
      require("arduino-nvim").setup()
    end,
  },
}
