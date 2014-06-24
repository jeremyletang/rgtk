// This file is part of rgtk.
//
// rgtk is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// rgtk is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License
// along with rgtk.  If not, see <http://www.gnu.org/licenses/>.

pub use self::rt::{
	init,
	main,
	main_quit,
	main_level,
	main_iteration,
	main_iteration_do,
	get_major_version,
	get_minor_version,
	get_micro_version,
	get_binary_age,
	get_interface_age,
	check_version
};

pub use self::widgets::{
	Window,
	Label,
	Button,
	_Box,
	ButtonBox,
	Frame,
	AspectFrame,
	Fixed,
	Separator,
	FontButton,
	ToggleButton,
	CheckButton,
	MenuButton,
	ColorButton,
	LinkButton,
	Adjustment,
	ScaleButton,
	VolumeButton,
	Grid,
	EntryBuffer,
	Entry,
	SearchEntry,
	Switch,
	Scale,
	LevelBar,
	SearchBar,
	SpinButton,
	Spinner,
	Image,
	ProgressBar,
	Arrow,
	Calendar,
	Alignment,
	Expander,
	Paned,
	InfoBar,
	Toolbar,
	ToolItem,
	SeparatorToolItem,
	ToolButton,
	ToggleToolButton,
	MenuToolButton,
	DrawingArea
};

pub use self::enums::{
	window_type,
	WindowType,
	text_direction,
	TextDirection,
	window_position,
	WindowPosition,
	button_box_style,
	ButtonBoxStyle,
	orientation,
	Orientation,
	direction_type,
	DirectionType,
	corner_type,
	CornerType,
	resize_mode,
	ResizeMode,
	border_style,
	BorderStyle,
	sort_type,
	SortType,
	state_flags,
	StateFlags,
	drag_result,
	DragResult,
	accel_flags,
	AccelFlags,
	arrow_placement,
	ArrowPlacement,
	arrow_type,
	ArrowType,
	attach_options,
	AttachOptions,
	delete_type,
	DeleteType,
	expander_style,
	ExpanderStyle,
	im_preedit_style,
	IMPreeditStyle,
	im_status_style,
	IMStatusStyle,
	justification,
	Justification,
	movement_step,
	MovementStep,
	pack_type,
	PackType,
	path_priority_type,
	PathPriorityType,
	path_type,
	PathType,
	policy_type,
	PolicyType,
	position_type,
	PositionType,
	relief_style,
	ReliefStyle,
	scroll_step,
	ScrollStep,
	scroll_type,
	ScrollType,
	selection_mode,
	SelectionMode,
	shadow_type,
	ShadowType,
	state_type,
	StateType,
	toolbar_style,
	ToolbarStyle,
	junction_sides,
	JunctionSides,
	region_flags,
	RegionFlags,
	icon_size,
	IconSize,
	entry_icon_position,
	EntryIconPosition,
	input_hints,
	InputHints,
	input_purpose,
	InputPurpose,
	image_type,
	ImageType,
	spin_type,
	SpinType,
	spin_button_update_policy,
	SpinButtonUpdatePolicy,
	level_bar_mode,
	LevelBarMode,
	calendar_display_options,
	CalendarDisplayOptions,
	message_type,
	MessageType
};

pub use self::types::{
	Tooltip,
	WidgetHelpType
};

pub mod traits;
pub mod signals;

mod macros;
mod cast;
mod rt;
mod widgets;
mod enums;
mod types;
mod ffi;