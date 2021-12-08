/* Copyright 2021 The Open Voice Network
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use argon2;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Record {

    #[serde(rename = "_id")]
    pub id: Uuid,
    pub domain_url: String,
    pub invocation_name: String,
    pub organization_name: String,
    pub organization_registrant: String,
    pub organization_email: String,
    pub organization_address: String,
    pub organization_city: String,
    pub organization_country: String,
    pub registered_date: DateTime<Utc>,
    pub expiration_date: DateTime<Utc>,
    pub location: String,
    pub destination_url: String,
    pub status: String,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InsertableRecord {
    pub domain_url: String,
    pub invocation_name: String,
    pub organization_name: String,
    pub organization_registrant: String,
    pub organization_email: String,
    pub organization_address: String,
    pub organization_city: String,
    pub organization_country: String,
    pub registered_date: DateTime<Utc>,
    pub expiration_date: DateTime<Utc>,
    pub location: String,
    pub destination_url: String,
    pub status: String,
}

impl Record {
    pub fn new(domain_url:String, invocation_name:String, 
        organization_name: String, organization_registrant: String,organization_email: String, 
        organization_address: String, organization_city: String, organization_country: String,  
        registered_date: DateTime<Utc>, expiration_date: DateTime<Utc>, 
        location:String, destination_url:String, status:String) -> Self {

        Record {
            id: Uuid::new_v4(),
            domain_url,
            invocation_name,
            organization_name,
            organization_registrant,
            organization_email,
            organization_address,
            organization_city,
            organization_country,
            registered_date,
            expiration_date,
            location,
            destination_url,
            status,
            created: Utc::now(),
            updated: Utc::now(),
        }
    }
    pub fn from_insertable(insertable: InsertableRecord) -> Self {
        // Record::new(insertable.domain_url, insertable.organization_name,
        Record::new(insertable.domain_url, insertable.invocation_name, insertable.organization_name, 
            insertable.organization_registrant, insertable.organization_email, insertable.organization_address, insertable.organization_city, insertable.organization_country,
            insertable.registered_date, insertable.expiration_date,
            insertable.location, insertable.destination_url, insertable.status)
    }
    

    pub fn update_record(&mut self,  domain_url: &String,invocation_name: &String, organization_name: &String, 
        organization_registrant: &String, organization_email: &String, organization_address: &String,
        organization_city: &String, organization_country: &String,
        registered_date: &DateTime<Utc>, expiration_date: &DateTime<Utc>,
        location: &String,destination_url: &String, status: &String) -> Self {
        self.domain_url = domain_url.to_string();
        self.invocation_name = invocation_name.to_string();
        self.organization_name = organization_name.to_string();
        self.organization_registrant = organization_registrant.to_string();
        self.organization_email = organization_email.to_string();
        self.organization_address = organization_address.to_string();
        self.organization_city = organization_city.to_string();
        self.organization_country = organization_country.to_string();
        self.location = location.to_string();
        self.destination_url = destination_url.to_string();
        self.registered_date = *registered_date;
        self.expiration_date = *expiration_date;
        self.status = status.to_string();
        self.updated = Utc::now();
        self.to_owned()
    }

}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResponseRecord {
    pub id: String,
    pub domain_url: String,
    pub invocation_name: String,
    pub organization_name: String,
    pub organization_registrant: String,
    pub organization_email: String,
    pub organization_address: String,
    pub organization_city: String,
    pub organization_country: String,
    pub registered_date: DateTime<Utc>,
    pub expiration_date: DateTime<Utc>,
    pub location: String,
    pub destination_url: String,
    pub status: String,
}

impl ResponseRecord{
    pub fn from_record(record: &Record)-> Self {
        ResponseRecord{
            id: record.id.to_string(),
            domain_url: format!("{}", record.domain_url),
            invocation_name: format!("{}", record.invocation_name),
            organization_name: format!("{}", record.organization_name),
            organization_registrant: format!("{}", record.organization_registrant),
            organization_email: format!("{}", record.organization_email),
            organization_address: format!("{}", record.organization_address),
            organization_city: format!("{}", record.organization_city),
            organization_country: format!("{}", record.organization_country),
            registered_date: record.registered_date,
            expiration_date: record.expiration_date,
            location: format!("{}", record.location),
            destination_url: format!("{}", record.destination_url),
            status: format!("{}", record.status),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    #[serde(rename = "_id")]
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub hashed_password: String,
    pub salt: String,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InsertableUser {
    pub name: String,
    pub email: String,
    pub password: String,
}

impl User {
    pub fn new(name: String, email: String, password: String) -> Self {
        let salt: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(20)
            .collect();
        let hashed_password = hash_password(&password, &salt);

        User {
            id: Uuid::new_v4(),
            name,
            email,
            hashed_password,
            salt,
            created: Utc::now(),
            updated: Utc::now(),
        }
    }
    pub fn from_insertable(insertable: InsertableUser) -> Self {
        User::new(insertable.name, insertable.email, insertable.password)
    }
    pub fn match_password(&self, password: &String) -> bool {
        argon2::verify_encoded(&self.hashed_password, password.as_bytes()).unwrap()
    }
    pub fn update_password(&mut self, password: &String) -> Self {
        self.hashed_password = hash_password(password, &self.salt);
        self.updated = Utc::now();
        self.to_owned()
    }
    pub fn update_user(&mut self, name: &String, email: &String) -> Self {
        self.name = name.to_string();
        self.email = email.to_string();
        self.updated = Utc::now();
        self.to_owned()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResponseUser {
    pub id: String,
    pub name: String,
    pub email: String,
}

impl ResponseUser{
    pub fn from_user(user: &User)-> Self {
        ResponseUser{
            id: user.id.to_string(),
            name: format!("{}", user.name),
            email: format!("{}", user.email),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct  UserPassword {
    pub password: String,
    pub new_password: Option<String>,
}

fn hash_password(password: &String, salt: &String) -> String {
    let config = argon2::Config::default();
    argon2::hash_encoded(password.as_bytes(), salt.as_bytes(), &config).unwrap()
}

