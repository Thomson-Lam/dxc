return {
  -- Keep Snacks picker as the search backend, but preserve the Telescope-era
  -- keybindings from nvim-config-2026.
  {
    "folke/snacks.nvim",
    keys = {
      { "<leader>sh", function() Snacks.picker.help() end, desc = "[S]earch [H]elp" },
      { "<leader>sk", function() Snacks.picker.keymaps() end, desc = "[S]earch [K]eymaps" },
      { "<leader>sf", function() Snacks.picker.files() end, desc = "[S]earch [F]iles" },
      { "<leader>ss", function() Snacks.picker.pickers() end, desc = "[S]earch [S]elect Picker" },
      {
        "<leader>sw",
        function() Snacks.picker.grep_word() end,
        mode = { "n", "x" },
        desc = "[S]earch current [W]ord",
      },
      { "<leader>sg", function() Snacks.picker.grep() end, desc = "[S]earch by [G]rep" },
      { "<leader>sd", function() Snacks.picker.diagnostics() end, desc = "[S]earch [D]iagnostics" },
      { "<leader>sr", function() Snacks.picker.resume() end, desc = "[S]earch [R]esume" },
      { "<leader>s.", function() Snacks.picker.recent() end, desc = "[S]earch Recent Files" },
      { "<leader><leader>", function() Snacks.picker.buffers() end, desc = "Find existing buffers" },

      -- LSP symbol search bindings from the old Telescope config.
      { "<leader>ws", function() Snacks.picker.lsp_workspace_symbols() end, desc = "LSP Workspace Symbols" },
      { "<leader>ds", function() Snacks.picker.lsp_workspace_symbols() end, desc = "LSP Dynamic Workspace Symbols" },
      { "<leader>so", function() Snacks.picker.lsp_symbols() end, desc = "Open Current Document Symbols" },

      { "<leader>fm", function() Snacks.picker.marks() end, desc = "[F]ind Vim [M]arks" },
      { "<leader>/", function() Snacks.picker.lines() end, desc = "[/] Fuzzily search in current buffer" },
      { "<leader>s/", function() Snacks.picker.grep_buffers() end, desc = "[S]earch [/] in Open Files" },
      {
        "<leader>sN",
        function()
          Snacks.picker.files({ cwd = vim.fn.stdpath("config") })
        end,
        desc = "[S]earch [N]eovim files",
      },
    },
  },
}
