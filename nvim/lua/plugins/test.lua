return {
  {
    "vim-test/vim-test",
    dependencies = {
      "preservim/vimux",
    },
    keys = {
      { "<leader>tn", "<cmd>TestNearest<cr>", desc = "Test Nearest" },
      { "<leader>tf", "<cmd>TestFile<cr>", desc = "Test File" },
      { "<leader>ta", "<cmd>TestSuite<cr>", desc = "Test Suite" },
      { "<leader>tl", "<cmd>TestLast<cr>", desc = "Test Last" },
      { "<leader>tv", "<cmd>TestVisit<cr>", desc = "Test Visit" },
    },
    config = function()
      vim.g["test#strategy"] = "vimux"
    end,
  },
}
