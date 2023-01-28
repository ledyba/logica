use std::borrow::Cow;
use eframe::egui::{self, DragValue};
use egui_node_graph::{
  DataTypeTrait,
  GraphEditorState,
  InputParamKind,
  NodeDataTrait,
  NodeId,
  NodeResponse,
  NodeTemplateIter,
  NodeTemplateTrait,
  UserResponseTrait,
  WidgetValueTrait,
};

pub struct NodeData {
  template: NodeTemplate,
}

#[derive(PartialEq, Eq)]
pub enum DataType {
  Scalar,
}

#[derive(Copy, Clone, Debug)]
pub enum ValueType {
  Scalar { value: f32 },
}

impl Default for ValueType {
  fn default() -> Self {
    Self::Scalar { value: 0.0 }
  }
}

#[derive(Clone, Copy)]
pub enum NodeTemplate {
  MakeScalar,
  AddScalar,
  SubScalar,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Response {
  SetActiveNode(NodeId),
  ClearActiveNode,
}

#[derive(Default)]
pub struct GraphState {
  pub active_node: Option<NodeId>,
}


impl DataTypeTrait<GraphState> for DataType {
  fn data_type_color(&self, _user_state: &mut GraphState) -> egui::Color32 {
    match self {
      DataType::Scalar => egui::Color32::from_rgb(38, 109, 211),
    }
  }

  fn name(&self) -> Cow<'_, str> {
    match self {
      DataType::Scalar => Cow::Borrowed("scalar"),
    }
  }
}

impl NodeTemplateTrait for NodeTemplate {
  type NodeData = NodeData;
  type DataType = DataType;
  type ValueType = ValueType;
  type UserState = GraphState;

  fn node_finder_label(&self, _user_state: &mut Self::UserState) -> Cow<'_, str> {
    Cow::Borrowed(match self {
      NodeTemplate::MakeScalar => "New scalar",
      NodeTemplate::AddScalar => "Scalar add",
      NodeTemplate::SubScalar => "Scalar sub",
    })
  }

  fn node_graph_label(&self, _user_state: &mut Self::UserState) -> String {
    String::from(match self {
      NodeTemplate::MakeScalar => "Scalar",
      NodeTemplate::AddScalar => "Scalar add",
      NodeTemplate::SubScalar => "Scalar sub",
    })
  }

  fn user_data(&self, _user_state: &mut Self::UserState) -> Self::NodeData {
    NodeData { template: *self }
  }

  fn build_node(
    &self,
    graph: &mut Graph,
    _user_state: &mut Self::UserState,
    node_id: NodeId,
  ) {
    let input_scalar = |graph: &mut Graph, name: &str| {
      graph.add_input_param(
        node_id,
        name.to_string(),
        DataType::Scalar,
        ValueType::Scalar { value: 0.0 },
        InputParamKind::ConnectionOrConstant,
        true,
      );
    };

    let output_scalar = |graph: &mut Graph, name: &str| {
      graph.add_output_param(node_id, name.to_string(), DataType::Scalar);
    };

    match self {
      NodeTemplate::MakeScalar => {
        graph.add_input_param(
          node_id,
          "Value".to_string(),
          DataType::Scalar,
          ValueType::Scalar { value: 0.0 },
          InputParamKind::ConstantOnly,
          true,
        );
        output_scalar(graph, "out");
      },
      NodeTemplate::AddScalar => {
        input_scalar(graph, "A");
        input_scalar(graph, "B");
        output_scalar(graph, "out");
      },
      NodeTemplate::SubScalar => {
        input_scalar(graph, "A");
        input_scalar(graph, "B");
        output_scalar(graph, "out");
      },
    }
  }
}

pub struct AllNodeTemplates;
impl NodeTemplateIter for AllNodeTemplates {
  type Item = NodeTemplate;

  fn all_kinds(&self) -> Vec<Self::Item> {
    vec![
      NodeTemplate::MakeScalar,
      NodeTemplate::AddScalar,
      NodeTemplate::SubScalar,
    ]
  }
}

impl WidgetValueTrait for ValueType {
  type Response = Response;
  type UserState = GraphState;
  type NodeData = NodeData;
  fn value_widget(
    &mut self,
    param_name: &str,
    _node_id: NodeId,
    ui: &mut egui::Ui,
    _user_state: &mut GraphState,
    _node_data: &NodeData,
  ) -> Vec<Response> {

    match self {
      ValueType::Scalar { value } => {
        ui.horizontal(|ui| {
          ui.label(param_name);
          ui.add(DragValue::new(value));
        });
      }
    }
    // This allows you to return your responses from the inline widgets.
    Vec::new()
  }
}

impl UserResponseTrait for Response {}
impl NodeDataTrait for NodeData {
  type Response = Response;
  type UserState = GraphState;
  type DataType = DataType;
  type ValueType = ValueType;

  fn bottom_ui(
    &self,
    ui: &mut egui::Ui,
    node_id: NodeId,
    _graph: &Graph,
    user_state: &mut Self::UserState,
  ) -> Vec<NodeResponse<Response, NodeData>>
    where
      Response: UserResponseTrait,
  {
    let mut responses = vec![];
    let is_active = user_state
      .active_node
      .map(|id| id == node_id)
      .unwrap_or(false);

    if !is_active {
      if ui.button("👁 Set active").clicked() {
        responses.push(NodeResponse::User(Response::SetActiveNode(node_id)));
      }
    } else {
      let button =
        egui::Button::new(egui::RichText::new("👁 Active").color(egui::Color32::BLACK))
          .fill(egui::Color32::GOLD);
      if ui.add(button).clicked() {
        responses.push(NodeResponse::User(Response::ClearActiveNode));
      }
    }
    responses
  }
}

pub type Graph = egui_node_graph::Graph<NodeData, DataType, ValueType>;
pub type EditorState = GraphEditorState<NodeData, DataType, ValueType, NodeTemplate, GraphState>;
