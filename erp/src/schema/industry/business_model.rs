use grand_line::*;
use async_graphql::*;
use serde::{Serialize, Deserialize};

#[derive(Enum, Copy, Clone, Eq, PartialEq, Debug, Default, Serialize, Deserialize)]
pub enum BusinessModel {
    #[default]
    ProductBased, // Hàng hóa vật lý, kho bãi, chuỗi cung ứng

    ServiceBased, // Công việc tri thức, tư vấn, dịch vụ liên tục

    ProjectBased, // Sản phẩm một lần, dựa trên cột mốc

    ExperienceBased, // Trải nghiệm khách hàng, tiêu thụ ngay lập tức
}