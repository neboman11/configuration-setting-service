use crate::database;
use crate::error_handler::CustomError;
use crate::schema::configuration_settings;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[table_name = "configuration_settings"]
pub struct ConfigurationSetting {
    pub section: String,
    pub name: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Queryable)]
pub struct ConfigurationSettings {
    pub id: i32,
    pub section: String,
    pub name: String,
    pub value: String,
}

impl ConfigurationSettings {
    pub fn find_all() -> Result<Vec<Self>, CustomError> {
        let mut conn = database::connection()?;
        let configuration_settings =
            configuration_settings::table.load::<ConfigurationSettings>(&mut conn)?;
        Ok(configuration_settings)
    }

    pub fn get_section(section: String) -> Result<Vec<Self>, CustomError> {
        let mut conn = database::connection()?;
        let results = configuration_settings::table
            .filter(configuration_settings::section.eq(section))
            .load::<ConfigurationSettings>(&mut conn)?;
        Ok(results)
    }

    pub fn find(id: i32) -> Result<Self, CustomError> {
        let mut conn = database::connection()?;
        let configuration_setting = configuration_settings::table
            .filter(configuration_settings::id.eq(id))
            .first(&mut conn)?;
        Ok(configuration_setting)
    }

    pub fn find_by_section_and_name(section: String, name: String) -> Result<Self, CustomError> {
        let mut conn = database::connection()?;
        let configuration_setting = configuration_settings::table
            .filter(configuration_settings::section.eq(section))
            .filter(configuration_settings::name.eq(name))
            .first(&mut conn)?;
        Ok(configuration_setting)
    }

    pub fn create(configuration_setting: ConfigurationSetting) -> Result<Self, CustomError> {
        let mut conn = database::connection()?;
        let configuration_setting = ConfigurationSetting::from(configuration_setting);
        let configuration_setting = diesel::insert_into(configuration_settings::table)
            .values(configuration_setting)
            .get_result(&mut conn)?;
        Ok(configuration_setting)
    }

    pub fn update(
        id: i32,
        configuration_setting: ConfigurationSetting,
    ) -> Result<Self, CustomError> {
        let mut conn = database::connection()?;
        let configuration_setting = diesel::update(configuration_settings::table)
            .filter(configuration_settings::id.eq(id))
            .set(configuration_setting)
            .get_result(&mut conn)?;
        Ok(configuration_setting)
    }

    pub fn update_by_secion_and_name(
        section: String,
        name: String,
        configuration_setting: ConfigurationSetting,
    ) -> Result<Self, CustomError> {
        let mut conn = database::connection()?;
        let configuration_setting = diesel::update(configuration_settings::table)
            .filter(configuration_settings::section.eq(section))
            .filter(configuration_settings::name.eq(name))
            .set(configuration_setting)
            .get_result(&mut conn)?;
        Ok(configuration_setting)
    }

    pub fn delete(id: i32) -> Result<usize, CustomError> {
        let mut conn = database::connection()?;
        let res =
            diesel::delete(configuration_settings::table.filter(configuration_settings::id.eq(id)))
                .execute(&mut conn)?;
        Ok(res)
    }
}

impl ConfigurationSetting {
    fn from(configuration_setting: ConfigurationSetting) -> ConfigurationSetting {
        ConfigurationSetting {
            section: configuration_setting.section,
            name: configuration_setting.name,
            value: configuration_setting.value,
        }
    }
}
