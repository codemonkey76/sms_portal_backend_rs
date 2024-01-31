Api Specification for SMS Portal




create_customer (CustomerForCreate)
	name: String,
	sender_id: String

get_customer
	id: i64

list_customers
	filter: Option<Vec<CustomerFilter>>
	list_options: Option<ListOptions>

update_customer (CustomerForUpdate)
	name: Option<String>,
	sender_id: Option<String>

delete_customer
	id: i64


archive_message

create_contact
get_contact
list_contacts
update_contact
delete_contact

set_current_customer


reset_password

create_list
get_list
list_lists
update_list
delete_list

create_message
get_message
list_messages
update_message
delete_message
send_message

create_template
get_template
list_templates
update_template
delete_template

create_user
get_user
list_users
update_user
delete_user



models:
	contact
	customer
	user
	list
	message
	template

Database Model Tables:

customers
	id: i64, PK
	name: String (255),
	sender_id: String (255),
	is_active: bool

contacts
	id: i64, PK
	phone: String (255),
	first_name: String (255),
	last_name: String (255),
	company_name: String (255),
	customer_id: i64, FK

lists
	id: i64: PK
	name: String (255),
	customer_id: i64 PK

users
	id: i64, PK
	name: String (255),
	email: String (255),
	email_verified_at: 

messages:
    id: i64

    customer_id: i64,
    user_id: i64,

    body: String,
    num_segments: i64,
    from: String,
    to: String,
    status: String,
    sid: String,
    is_mms: bool,

    is_archived: bool

Pivot Tables:

customer_user
	id: i64, PK
	user_id: i64, FK
	customer_id: i64, FK

contact_list
	id: i64, PK
	contact_id: i64, FK
	list_id: i64, FK