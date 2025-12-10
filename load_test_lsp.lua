local client = vim.lsp.start_client({
	name = "cuelsp",
	cmd = { "/home/leaf/Dev/nudox/cuelsp/target/debug/cuelsp" },
})

if not client then
	vim.notify("DIDNT WORK BOOHOO")
	return
end

vim.api.nvim_create_autocmd("FileType", {
	pattern = "markdown",
	callback = function()
		vim.lsp.buf_attach_client(0, client)
	end,
})
