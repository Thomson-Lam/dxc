return {
  -- Keep nvim-current/LazyVim utility plugin configs for gitsigns,
  -- todo-comments, plenary, nui, lazy.nvim, lualine, mini.icons, etc.
  -- Add only the utility plugin from nvim-config-2026 that current lacks.
  {
    "NMAC427/guess-indent.nvim",
    event = "BufReadPre",
    opts = {},
  },
}
