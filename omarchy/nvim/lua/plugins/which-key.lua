return {
  {
    "folke/which-key.nvim",
    opts = {
      spec = {
        {
          mode = { "n", "x" },
          { "<leader>t", group = "test" },
          { "<leader>e", desc = "Toggle nvim-tree" },
          { "<leader>F", desc = "Toggle Neo-tree" },
        },
      },
    },
  },
}
