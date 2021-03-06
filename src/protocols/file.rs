use crate::config::Config;
use crate::protocols::PathType;
use crate::services;
use std::borrow::Cow;
use std::fmt::{self, Display, Formatter};
use std::path::Path;

pub struct File<'a> {
    pub path: Cow<'a, Path>,
    pub file_type: PathType,
    pub group: String,
    pub user: String,
    pub time: filetime::FileTime,
    pub size: u64,
    pub perms: String,
    pub config: &'a Config,
}

impl<'a> File<'a> {
    pub fn new(path: Cow<'a, Path>, config: &'a Config) -> anyhow::Result<Self> {
        let metadata = path.symlink_metadata()?;
        let time = services::time::get(&metadata, config.time.ty);
        Ok(Self {
            group: services::group(&metadata),
            user: services::user(&metadata),
            time,
            size: services::size::get(&metadata),
            perms: services::perms::perms(&metadata, &config.colors.perms),
            file_type: PathType::from_path(&path, Some(metadata))?,
            path,
            config,
        })
    }
    pub fn file_name(&self) -> std::borrow::Cow<'_, str> {
        self.path
            .file_name()
            .map(|path| path.to_string_lossy())
            .unwrap_or(std::borrow::Cow::Borrowed(""))
    }
}

impl Display for File<'_> {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        use crate::config::ViewFormat;
        use colored::Colorize;
        let file_name = self.file_name();
        write!(
            formatter,
            "{}{}",
            self.file_type
                .color(&self.config.colors.types)
                .apply(file_name.as_ref())
                .bold(),
            self.file_type
                .extra()
                .unwrap_or(colored::Colorize::normal("")),
        )?;
        let show_metadata = if matches!(self.file_type, PathType::Directory { .. }) {
            self.config.show_metadata.directory
        } else {
            self.config.show_metadata.file
        };
        match self.config.view_format {
            ViewFormat::Full if show_metadata => {
                struct FieldDisplay<'a>(&'a File<'a>);
                impl Display for FieldDisplay<'_> {
                    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
                        for (index, field) in self.0.config.field_order.iter().enumerate() {
                            if index != 0 {
                                std::fmt::Write::write_char(formatter, ' ')?;
                            }
                            write!(formatter, "{}", field.display(&self.0))?;
                        }
                        Ok(())
                    }
                }

                write!(
                    formatter,
                    " {open_bracket}{}{close_bracket}",
                    FieldDisplay(self),
                    open_bracket = "[".white().bold(),
                    close_bracket = "]".white().bold()
                )?;
                match self.file_type {
                    PathType::Directory { num_entries } => {
                        write!(
                            formatter,
                            "{}",
                            format!(" ({} items)", num_entries).white().bold()
                        )
                    }
                    _ => Ok(()),
                }
            }
            _ => Ok(()),
        }
    }
}
