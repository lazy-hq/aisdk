use base64::Engine;
use reqwest::{Client, Response};
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attribute {
    pub schema_id: String,
    pub name: String,
    pub description: String,
    pub friendly_id: String,
    pub handle: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_product_count: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Category {
    pub schema_id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_product_count: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryWithChildren {
    pub schema_id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_product_count: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValueOption {
    pub schema_id: String,
    pub name: String,
    pub schema_friendly_id: String,
    pub handle: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    pub count: i32,
    pub next: Option<String>,
    pub previous: Option<String>,
    pub results: Vec<T>,
}

pub struct TaxonomyApiClient {
    client: Client,
    base_url: String,
    auth_header: Option<String>,
}

impl TaxonomyApiClient {
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            client: Client::new(),
            base_url: base_url.into(),
            auth_header: None,
        }
    }

    pub fn with_basic_auth(mut self, username: &str, password: &str) -> Self {
        let credentials = format!("{}:{}", username, password);
        let encoded = base64::engine::general_purpose::STANDARD.encode(credentials.as_bytes());
        self.auth_header = Some(format!("Basic {}", encoded));
        self
    }

    fn build_url(&self, path: &str, params: &[(&str, String)]) -> String {
        let mut url = format!("{}{}", self.base_url, path);
        if !params.is_empty() {
            url.push('?');
            let query_string = params
                .iter()
                .map(|(k, v)| format!("{}={}", k, urlencoding::encode(v)))
                .collect::<Vec<_>>()
                .join("&");
            url.push_str(&query_string);
        }
        url
    }

    async fn send_request(&self, url: &str) -> Result<Response, Box<dyn Error>> {
        let mut request = self.client.get(url);
        if let Some(auth) = &self.auth_header {
            request = request.header("Authorization", auth);
        }
        Ok(request.send().await?)
    }

    pub fn list_attributes(&self) -> AttributeListBuilder<'_> {
        AttributeListBuilder::new(self)
    }

    pub async fn get_attribute(&self, schema_id: &str) -> Result<Attribute, Box<dyn Error>> {
        let url = format!("{}/attribute/{}/", self.base_url, schema_id);
        let response = self.send_request(&url).await?;
        Ok(response.json().await?)
    }

    pub fn list_categories(&self) -> CategoryListBuilder<'_> {
        CategoryListBuilder::new(self)
    }

    pub fn get_category(&self, schema_id: &str) -> CategoryGetBuilder<'_> {
        CategoryGetBuilder::new(self, schema_id)
    }

    pub fn list_value_options(&self) -> ValueOptionListBuilder<'_> {
        ValueOptionListBuilder::new(self)
    }
}

pub struct AttributeListBuilder<'a> {
    client: &'a TaxonomyApiClient,
    name: Option<String>,
    category: Option<String>,
    category_schema_id: Option<String>,
    limit: Option<i32>,
    offset: Option<i32>,
}

impl<'a> AttributeListBuilder<'a> {
    fn new(client: &'a TaxonomyApiClient) -> Self {
        Self {
            client,
            name: None,
            category: None,
            category_schema_id: None,
            limit: None,
            offset: None,
        }
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn category(mut self, category: impl Into<String>) -> Self {
        self.category = Some(category.into());
        self
    }

    pub fn category_schema_id(mut self, category_schema_id: impl Into<String>) -> Self {
        self.category_schema_id = Some(category_schema_id.into());
        self
    }

    pub fn limit(mut self, limit: i32) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn offset(mut self, offset: i32) -> Self {
        self.offset = Some(offset);
        self
    }

    pub async fn send(self) -> Result<PaginatedResponse<Attribute>, Box<dyn Error>> {
        let mut params = Vec::new();
        if let Some(name) = self.name {
            params.push(("name", name));
        }
        if let Some(category) = self.category {
            params.push(("category", category));
        }
        if let Some(category_schema_id) = self.category_schema_id {
            params.push(("category__schema_id", category_schema_id));
        }
        if let Some(limit) = self.limit {
            params.push(("limit", limit.to_string()));
        }
        if let Some(offset) = self.offset {
            params.push(("offset", offset.to_string()));
        }

        let url = self.client.build_url("/client/attribute/", &params);
        let response = self.client.send_request(&url).await?;
        Ok(response.json().await?)
    }
}

pub struct CategoryListBuilder<'a> {
    client: &'a TaxonomyApiClient,
    name: Option<String>,
    name_icontains: Option<String>,
    parent: Option<String>,
    schema_id: Option<String>,
    parent_isnull: Option<String>,
    parent_schema_id: Option<String>,
    with_products: Option<String>,
    limit: Option<i32>,
    offset: Option<i32>,
}

impl<'a> CategoryListBuilder<'a> {
    fn new(client: &'a TaxonomyApiClient) -> Self {
        Self {
            client,
            name: None,
            name_icontains: None,
            parent: None,
            schema_id: None,
            parent_isnull: None,
            parent_schema_id: None,
            with_products: None,
            limit: None,
            offset: None,
        }
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn name_icontains(mut self, name_icontains: impl Into<String>) -> Self {
        self.name_icontains = Some(name_icontains.into());
        self
    }

    pub fn parent(mut self, parent: impl Into<String>) -> Self {
        self.parent = Some(parent.into());
        self
    }

    pub fn schema_id(mut self, schema_id: impl Into<String>) -> Self {
        self.schema_id = Some(schema_id.into());
        self
    }

    pub fn parent_isnull(mut self, parent_isnull: impl Into<String>) -> Self {
        self.parent_isnull = Some(parent_isnull.into());
        self
    }

    pub fn parent_schema_id(mut self, parent_schema_id: impl Into<String>) -> Self {
        self.parent_schema_id = Some(parent_schema_id.into());
        self
    }

    pub fn with_products(mut self, with_products: impl Into<String>) -> Self {
        self.with_products = Some(with_products.into());
        self
    }

    pub fn limit(mut self, limit: i32) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn offset(mut self, offset: i32) -> Self {
        self.offset = Some(offset);
        self
    }

    pub async fn send(self) -> Result<PaginatedResponse<Category>, Box<dyn Error>> {
        let mut params = Vec::new();
        if let Some(name) = self.name {
            params.push(("name", name));
        }
        if let Some(name_icontains) = self.name_icontains {
            params.push(("name__icontains", name_icontains));
        }
        if let Some(parent) = self.parent {
            params.push(("parent", parent));
        }
        if let Some(schema_id) = self.schema_id {
            params.push(("schema_id", schema_id));
        }
        if let Some(parent_isnull) = self.parent_isnull {
            params.push(("parent_isnull", parent_isnull));
        }
        if let Some(parent_schema_id) = self.parent_schema_id {
            params.push(("parent__schema_id", parent_schema_id));
        }
        if let Some(with_products) = self.with_products {
            params.push(("with_products", with_products));
        }
        if let Some(limit) = self.limit {
            params.push(("limit", limit.to_string()));
        }
        if let Some(offset) = self.offset {
            params.push(("offset", offset.to_string()));
        }

        let url = self.client.build_url("/client/category/", &params);
        let response = self.client.send_request(&url).await?;
        Ok(response.json().await?)
    }
}

pub struct CategoryGetBuilder<'a> {
    client: &'a TaxonomyApiClient,
    schema_id: String,
    with_products: Option<String>,
}

impl<'a> CategoryGetBuilder<'a> {
    fn new(client: &'a TaxonomyApiClient, schema_id: impl Into<String>) -> Self {
        Self {
            client,
            schema_id: schema_id.into(),
            with_products: None,
        }
    }

    pub fn with_products(mut self, with_products: impl Into<String>) -> Self {
        self.with_products = Some(with_products.into());
        self
    }

    pub async fn send(self) -> Result<CategoryWithChildren, Box<dyn Error>> {
        let mut params = Vec::new();
        if let Some(with_products) = self.with_products {
            params.push(("with_products", with_products));
        }

        let path = format!("/client/category/{}/", self.schema_id);
        let url = self.client.build_url(&path, &params);
        let response = self.client.send_request(&url).await?;
        Ok(response.json().await?)
    }
}

pub struct ValueOptionListBuilder<'a> {
    client: &'a TaxonomyApiClient,
    schema_id: Option<String>,
    name: Option<String>,
    schema_friendly_id: Option<String>,
    handle: Option<String>,
    attribute: Option<String>,
    attribute_schema_id: Option<String>,
    limit: Option<i32>,
    offset: Option<i32>,
}

impl<'a> ValueOptionListBuilder<'a> {
    fn new(client: &'a TaxonomyApiClient) -> Self {
        Self {
            client,
            schema_id: None,
            name: None,
            schema_friendly_id: None,
            handle: None,
            attribute: None,
            attribute_schema_id: None,
            limit: None,
            offset: None,
        }
    }

    pub fn schema_id(mut self, schema_id: impl Into<String>) -> Self {
        self.schema_id = Some(schema_id.into());
        self
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn schema_friendly_id(mut self, schema_friendly_id: impl Into<String>) -> Self {
        self.schema_friendly_id = Some(schema_friendly_id.into());
        self
    }

    pub fn handle(mut self, handle: impl Into<String>) -> Self {
        self.handle = Some(handle.into());
        self
    }

    pub fn attribute(mut self, attribute: impl Into<String>) -> Self {
        self.attribute = Some(attribute.into());
        self
    }

    pub fn attribute_schema_id(mut self, attribute_schema_id: impl Into<String>) -> Self {
        self.attribute_schema_id = Some(attribute_schema_id.into());
        self
    }

    pub fn limit(mut self, limit: i32) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn offset(mut self, offset: i32) -> Self {
        self.offset = Some(offset);
        self
    }

    pub async fn send(self) -> Result<PaginatedResponse<ValueOption>, Box<dyn Error>> {
        let mut params = Vec::new();
        if let Some(schema_id) = self.schema_id {
            params.push(("schema_id", schema_id));
        }
        if let Some(name) = self.name {
            params.push(("name", name));
        }
        if let Some(schema_friendly_id) = self.schema_friendly_id {
            params.push(("schema_friendly_id", schema_friendly_id));
        }
        if let Some(handle) = self.handle {
            params.push(("handle", handle));
        }
        if let Some(attribute) = self.attribute {
            params.push(("attribute", attribute));
        }
        if let Some(attribute_schema_id) = self.attribute_schema_id {
            params.push(("attribute__schema_id", attribute_schema_id));
        }
        if let Some(limit) = self.limit {
            params.push(("limit", limit.to_string()));
        }
        if let Some(offset) = self.offset {
            params.push(("offset", offset.to_string()));
        }

        let url = self.client.build_url("/client/value-option/", &params);
        let response = self.client.send_request(&url).await?;
        Ok(response.json().await?)
    }
}
