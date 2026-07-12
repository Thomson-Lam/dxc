return {
  -- File explorers: nvim-tree, neo-tree, and oil.
  {
    "nvim-neo-tree/neo-tree.nvim",
    cmd = "Neotree",
    keys = {
      {
        "<leader>F",
        function()
          require("neo-tree.command").execute({ toggle = true, dir = LazyVim.root(), position = "left" })
        end,
        desc = "Explorer NeoTree (Root Dir)",
      },
    },
    opts = {
      sources = { "filesystem", "buffers", "git_status" },
      open_files_do_not_replace_types = { "terminal", "Trouble", "trouble", "qf", "Outline" },
      filesystem = {
        bind_to_cwd = false,
        follow_current_file = { enabled = true },
        hijack_netrw_behavior = "disabled",
        use_libuv_file_watcher = true,
      },
      window = {
        position = "left",
        mappings = {
          ["l"] = "open",
          ["h"] = "close_node",
          ["<space>"] = "none",
        },
      },
      default_component_configs = {
        indent = {
          with_expanders = true,
          expander_collapsed = "",
          expander_expanded = "",
          expander_highlight = "NeoTreeExpander",
        },
        git_status = {
          symbols = {
            unstaged = "󰄱",
            staged = "󰱒",
          },
        },
      },
    },
  },

  {
    "nvim-tree/nvim-tree.lua",
    cmd = {
      "NvimTreeToggle",
      "NvimTreeOpen",
      "NvimTreeClose",
      "NvimTreeFindFile",
    },
    keys = {
      { "<leader>e", "<cmd>NvimTreeToggle<cr>", desc = "Toggle file tree" },
    },
    init = function()
      -- nvim-tree recommends disabling netrw before setup.
      vim.g.loaded_netrw = 1
      vim.g.loaded_netrwPlugin = 1
    end,
    opts = {
      disable_netrw = true,
      hijack_netrw = true,
      hijack_directories = {
        enable = false,
        auto_open = false,
      },
      respect_buf_cwd = true,
      sync_root_with_cwd = true,
      view = {
        relativenumber = true,
        float = {
          enable = true,
          open_win_config = function()
            local screen_w = vim.opt.columns:get()
            local screen_h = vim.opt.lines:get() - vim.opt.cmdheight:get()
            local window_w = screen_w * 0.5
            local window_h = screen_h * 0.8
            local window_w_int = math.floor(window_w)
            local window_h_int = math.floor(window_h)
            local center_x = (screen_w - window_w) / 2
            local center_y = ((vim.opt.lines:get() - window_h) / 2) - vim.opt.cmdheight:get()
            return {
              border = "rounded",
              relative = "editor",
              row = center_y,
              col = center_x,
              width = window_w_int,
              height = window_h_int,
            }
          end,
        },
        width = function()
          return math.floor(vim.opt.columns:get() * 0.8)
        end,
      },
    },
  },

  {
    "stevearc/oil.nvim",
    lazy = false,
    cmd = "Oil",
    keys = {
      { "<C-o>", "<cmd>Oil<cr>", desc = "Open Oil" },
    },
    opts = {
      default_file_explorer = true,
      use_default_keymaps = true,
    },
    config = function(_, opts)
      require("oil").setup(opts)

      vim.api.nvim_create_autocmd("VimEnter", {
        once = true,
        callback = function()
          if vim.fn.argc() == 1 then
            local arg = vim.fn.argv(0)
            if vim.fn.isdirectory(arg) == 1 then
              vim.schedule(function()
                require("oil").open(arg)
              end)
            end
          end
        end,
      })
    end,
  },

  {
    "akinsho/bufferline.nvim",
    keys = {
      { "<S-l>", false },
      { "<leader>bn", "<cmd>BufferLineCycleNext<cr>", desc = "Next Buffer" },
    },
  },

  {
    "leath-dub/snipe.nvim",
    keys = {
      {
        "<S-l>",
        function()
          require("snipe").open_buffer_menu()
        end,
        desc = "Open Snipe buffer menu",
      },
    },
    opts = {
      hints = {
        dictionary = "asfghl;wertyuiop",
      },
      navigate = {
        close_buffer = "d",
      },
    },
  },

  {
    "christoomey/vim-tmux-navigator",
    lazy = false,
    cmd = {
      "TmuxNavigateLeft",
      "TmuxNavigateDown",
      "TmuxNavigateUp",
      "TmuxNavigateRight",
      "TmuxNavigatePrevious",
      "TmuxNavigatorProcessList",
    },
    keys = {
      { "<c-h>", "<cmd><C-U>TmuxNavigateLeft<cr>", desc = "Tmux Navigate Left" },
      { "<c-j>", "<cmd><C-U>TmuxNavigateDown<cr>", desc = "Tmux Navigate Down" },
      { "<c-k>", "<cmd><C-U>TmuxNavigateUp<cr>", desc = "Tmux Navigate Up" },
      { "<c-l>", "<cmd><C-U>TmuxNavigateRight<cr>", desc = "Tmux Navigate Right" },
      { "<c-\\>", "<cmd><C-U>TmuxNavigatePrevious<cr>", desc = "Tmux Navigate Previous" },
    },
  },
}
