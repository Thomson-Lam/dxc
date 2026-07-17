local blink_review = vim.env.BLINK_MODE == "slow" or vim.env.BLINK_MODE == "blitz"

return {
	"folke/snacks.nvim",
	opts = {
		scroll = {
			enabled = false, -- Disable scrolling animations
		},
		notifier = {
			top_down = not blink_review,
			margin = {
				top = 0,
				right = 1,
				bottom = 0,
			},
		},
	},
}
