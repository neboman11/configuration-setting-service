use crate::database;
use crate::error_handler::CustomError;
use crate::schema::configuration_settings;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, AsChangeset, Insertable, ToSchema)]
#[diesel(table_name = configuration_settings)]
pub struct ConfigurationSetting {
    pub section: String,
    pub name: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name = configuration_settings)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ConfigurationSettings {
    pub id: i32,
    pub section: String,
    pub name: String,
    pub value: String,
}

impl ConfigurationSettings {
    pub fn find_all() -> Result<Vec<Self>, CustomError> {
        let mut conn = database::connection()?;
        let configuration_settings = configuration_settings::table
            .select(ConfigurationSettings::as_select())
            .load(&mut conn)?;
        Ok(configuration_settings)
    }

    pub fn get_section(section: String) -> Result<Vec<Self>, CustomError> {
        let mut conn = database::connection()?;
        let results = configuration_settings::table
            .filter(configuration_settings::section.eq(section))
            .select(ConfigurationSettings::as_select())
            .load(&mut conn)?;
        Ok(results)
    }

    pub fn find(id: i32) -> Result<Self, CustomError> {
        let mut conn = database::connection()?;
        let configuration_setting = configuration_settings::table
            .filter(configuration_settings::id.eq(id))
            .select(ConfigurationSettings::as_select())
            .first(&mut conn)?;
        Ok(configuration_setting)
    }

    pub fn find_by_section_and_name(section: String, name: String) -> Result<Self, CustomError> {
        let mut conn = database::connection()?;
        let configuration_setting = configuration_settings::table
            .filter(configuration_settings::section.eq(section))
            .filter(configuration_settings::name.eq(name))
            .select(ConfigurationSettings::as_select())
            .first(&mut conn)?;
        Ok(configuration_setting)
    }

    pub fn create(configuration_setting: ConfigurationSetting) -> Result<Self, CustomError> {
        let mut conn = database::connection()?;
        let configuration_setting = diesel::insert_into(configuration_settings::table)
            .values(configuration_setting)
            .returning(ConfigurationSettings::as_returning())
            .get_result(&mut conn)?;
        Ok(configuration_setting)
    }

    pub fn update(id: i32, configuration_setting: ConfigurationSetting) -> Result<Self, CustomError> {
        let mut conn = database::connection()?;
        let configuration_setting = diesel::update(configuration_settings::table)
            .filter(configuration_settings::id.eq(id))
            .set(configuration_setting)
            .returning(ConfigurationSettings::as_returning())
            .get_result(&mut conn)?;
        Ok(configuration_setting)
    }

    pub fn update_by_section_and_name(
        section: String,
        name: String,
        configuration_setting: ConfigurationSetting,
    ) -> Result<Self, CustomError> {
        let mut conn = database::connection()?;
        let configuration_setting = diesel::update(configuration_settings::table)
            .filter(configuration_settings::section.eq(section))
            .filter(configuration_settings::name.eq(name))
            .set(configuration_setting)
            .returning(ConfigurationSettings::as_returning())
            .get_result(&mut conn)?;
        Ok(configuration_setting)
    }

    pub fn delete(id: i32) -> Result<usize, CustomError> {
        let mut conn = database::connection()?;
        let res = diesel::delete(
            configuration_settings::table.filter(configuration_settings::id.eq(id)),
        )
        .execute(&mut conn)?;
        Ok(res)
    }
}
