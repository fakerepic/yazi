local M = {}

function M:peek()
	local folder = Folder:by_kind(Folder.PREVIEW)
	if not folder or folder.cwd ~= self.file.url then
		return {}
	end

	local bound = math.max(0, #folder.files - self.area.h)
	if self.skip > bound then
		ya.manager_emit("peek", { tostring(bound), only_if = tostring(self.file.url), upper_bound = "" })
	end

	local items, markers = {}, {}
	for i, f in ipairs(folder.window) do
		-- Highlight hovered file
		local item = ui.ListItem(ui.Line { Folder:icon(f), ui.Span(f.name) })
		if f:is_hovered() then
			item = item:style(THEME.manager.preview_hovered)
		else
			item = item:style(f:style())
		end
		items[#items + 1] = item

		-- Yanked/marked/selected files
		local marker = Folder:marker(f)
		if marker ~= 0 then
			markers[#markers + 1] = { i, marker }
		end
	end

	ya.preview_widgets(
		self,
		ya.flat {
			ui.List(self.area, items),
			Folder:markers(self.area, markers),
		}
	)
end

function M:seek(units)
	local folder = Folder:by_kind(Folder.PREVIEW)
	if folder and folder.cwd == self.file.url then
		local step = math.floor(units * self.area.h / 10)
		local bound = math.max(0, #folder.files - self.area.h)
		ya.manager_emit("peek", {
			tostring(ya.clamp(0, cx.active.preview.skip + step, bound)),
			only_if = tostring(self.file.url),
		})
	end
end

return M
