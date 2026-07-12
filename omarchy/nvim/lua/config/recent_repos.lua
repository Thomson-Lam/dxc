local M = {}

local defaults = {
  max_oldfiles = 200,
  max_repos = 30,
}

local function git_root(dir)
  local result = vim.fn.systemlist({ "git", "-C", dir, "rev-parse", "--show-toplevel" })
  if vim.v.shell_error ~= 0 or not result[1] or result[1] == "" then
    return nil
  end
  return vim.fn.fnamemodify(result[1], ":p"):gsub("/$", "")
end

function M.list(opts)
  opts = vim.tbl_extend("force", defaults, opts or {})

  local repos = {}
  local seen_repos = {}
  local checked_dirs = {}
  local oldfiles = vim.v.oldfiles or {}

  for i, file in ipairs(oldfiles) do
    if i > opts.max_oldfiles or #repos >= opts.max_repos then
      break
    end

    local path = vim.fn.fnamemodify(file, ":p")
    local dir = vim.fn.isdirectory(path) == 1 and path or vim.fn.fnamemodify(path, ":h")

    if dir ~= "" and not checked_dirs[dir] and vim.fn.isdirectory(dir) == 1 then
      checked_dirs[dir] = true

      local root = git_root(dir)
      if root and not seen_repos[root] then
        seen_repos[root] = true
        repos[#repos + 1] = root
      end
    end
  end

  return repos
end

function M.select(opts, on_choice)
  local repos = M.list(opts)

  if #repos == 0 then
    vim.notify("No recent Git repos found", vim.log.levels.INFO, { title = "Recent Git Repos" })
    return
  end

  vim.ui.select(repos, {
    prompt = "Recent Git Repos",
    format_item = function(repo)
      return vim.fn.fnamemodify(repo, ":~")
    end,
  }, function(repo)
    if repo then
      on_choice(repo)
    end
  end)
end

function M.pick(opts)
  M.select(opts, function(repo)
    vim.cmd.cd(vim.fn.fnameescape(repo))
    require("oil").open(repo)
  end)
end

function M.tmux_workflow(opts)
  M.select(opts, function(repo)
    vim.cmd.cd(vim.fn.fnameescape(repo))
    vim.cmd.tabnew()
    vim.fn.termopen({ "zsh", "-ic", "td" }, { cwd = repo })
    vim.cmd.startinsert()
  end)
end

return M
