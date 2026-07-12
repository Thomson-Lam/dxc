return {
  {
    "folke/snacks.nvim",
    opts = {
      dashboard = {
        preset = {
          header = [[
 █████╗ ██████╗ ███╗   ███╗ █████╗ ██╗   ██╗██╗███╗   ███╗
██╔══██╗██╔══██╗████╗ ████║██╔══██╗██║   ██║██║████╗ ████║
███████║██████╔╝██╔████╔██║███████║██║   ██║██║██╔████╔██║
██╔══██║██╔══██╗██║╚██╔╝██║██╔══██║╚██╗ ██╔╝██║██║╚██╔╝██║
██║  ██║██║  ██║██║ ╚═╝ ██║██║  ██║ ╚████╔╝ ██║██║ ╚═╝ ██║
╚═╝  ╚═╝╚═╝  ╚═╝╚═╝     ╚═╝╚═╝  ╚═╝  ╚═══╝  ╚═╝╚═╝     ╚═╝
          ]],
          keys = {
            { icon = " ", key = "f", desc = "Find File", action = ":lua Snacks.dashboard.pick('files')" },
            { icon = " ", key = "n", desc = "New File", action = ":ene | startinsert" },
            { icon = " ", key = "g", desc = "Find Text", action = ":lua Snacks.dashboard.pick('live_grep')" },
            { icon = " ", key = "r", desc = "Recent Files", action = ":lua Snacks.dashboard.pick('oldfiles')" },
            { icon = " ", key = "p", desc = "Recent Git Repos", action = ":lua require('config.recent_repos').pick()" },
            { icon = " ", key = "w", desc = "Tmux Workflow", action = ":lua require('config.recent_repos').tmux_workflow()" },
            {
              icon = " ",
              key = "c",
              desc = "Config",
              action = ":lua Snacks.dashboard.pick('files', {cwd = vim.fn.stdpath('config')})",
            },
            { icon = "󰒲 ", key = "l", desc = "Lazy", action = ":Lazy" },
            { icon = " ", key = "q", desc = "Quit", action = ":qa" },
          },
        },
      },
    },
  },
}
