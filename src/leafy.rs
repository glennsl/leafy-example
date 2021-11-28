#![allow(unused)]

use seed::{prelude::*, *};
use std::fmt;
use std::rc::Rc;

// Map

pub struct Map<'a, Ms> {
    key: Option<&'a str>,
    lat: Option<f64>,
    lng: Option<f64>,
    zoom: Option<i32>,
    features: Vec<Box<dyn Feature<Ms>>>,
}

pub fn map<'a, Ms>() -> Map<'a, Ms> {
    Map {
        key: None,
        lat: None,
        lng: None,
        zoom: None,
        features: vec![],
    }
}

impl<'a, Ms> Map<'a, Ms> {
    pub fn with(mut self, feature: impl Feature<Ms> + 'static) -> Self {
        self.features.push(Box::new(feature));
        self
    }

    pub fn key(mut self, key: &'a str) -> Self {
        self.key = Some(key);
        self
    }

    pub fn center_on(mut self, lat: f64, lng: f64) -> Self {
        self.lat = Some(lat);
        self.lng = Some(lng);
        self
    }

    pub fn zoom(mut self, zoom: i32) -> Self {
        self.zoom = Some(zoom);
        self
    }

    pub fn into_node(&'a self) -> Node<Ms> {
        custom![
            Tag::from("leafy-map"),
            self.key.as_ref().map(el_key),
            attrs! {
                At::from("lat") => self.lat.as_at_value(),
                At::from("lng") => self.lng.as_at_value(),
                At::from("zoom") => self.zoom.as_at_value(),
            },
            self.features.iter().map(|f| f.into_node()),
        ]
    }

    pub fn into_nodes(&self) -> Vec<Node<Ms>> {
        nodes![self.into_node()]
    }
}

impl<'a, Ms> UpdateEl<Ms> for Map<'a, Ms> {
    fn update_el(self, el: &mut El<Ms>) {
        el.children.append(&mut self.into_nodes())
    }
}

// Feature

pub trait Feature<Ms> {
    fn into_node(&self) -> Node<Ms>;
}

// FeatureGroup

pub struct FeatureGroup<Ms> {
    zoom_to_fit: Option<bool>,
    features: Vec<Box<dyn Feature<Ms>>>,
}

pub fn feature_group<Ms>() -> FeatureGroup<Ms> {
    FeatureGroup {
        zoom_to_fit: None,
        features: vec![],
    }
}

impl<Ms> FeatureGroup<Ms> {
    pub fn with(mut self, feature: impl Feature<Ms> + 'static) -> Self {
        self.features.push(Box::new(feature));
        self
    }

    pub fn zoom_to_fit(mut self) -> Self {
        self.zoom_to_fit = Some(true);
        self
    }
}

impl<Ms> Feature<Ms> for FeatureGroup<Ms> {
    fn into_node(&self) -> Node<Ms> {
        custom![
            Tag::from("leafy-feature-group"),
            attrs! {
                At::from("zoom-to-fit") => self.zoom_to_fit.as_at_value(),
            },
            self.features.iter().map(|f| f.into_node()),
        ]
    }
}

// Marker

pub struct Marker<Ms> {
    lat: f64,
    lng: f64,
    tooltip: Option<String>,
    tooltip_open: Option<bool>,
    on_click: Option<Rc<dyn Fn() -> Ms>>,
    on_mouseover: Option<Rc<dyn Fn() -> Ms>>,
    on_mouseout: Option<Rc<dyn Fn() -> Ms>>,
}

pub fn marker<Ms>(lat: f64, lng: f64) -> Marker<Ms> {
    Marker {
        lat,
        lng,
        tooltip: None,
        tooltip_open: None,
        on_click: None,
        on_mouseover: None,
        on_mouseout: None,
    }
}

impl<Ms> Marker<Ms> {
    pub fn tooltip<S: Into<String>>(mut self, value: S) -> Self {
        self.tooltip = Some(value.into());
        self
    }

    pub fn tooltip_open(mut self, value: bool) -> Self {
        self.tooltip_open = Some(value);
        self
    }

    pub fn on_click<F>(mut self, handler: F) -> Self
    where
        F: FnOnce() -> Ms + Clone + 'static,
    {
        self.on_click = Some(Rc::new(move || handler.clone()()));
        self
    }

    pub fn on_mouseover<F>(mut self, handler: F) -> Self
    where
        F: FnOnce() -> Ms + Clone + 'static,
    {
        self.on_mouseover = Some(Rc::new(move || handler.clone()()));
        self
    }

    pub fn on_mouseout<F>(mut self, handler: F) -> Self
    where
        F: FnOnce() -> Ms + Clone + 'static,
    {
        self.on_mouseout = Some(Rc::new(move || handler.clone()()));
        self
    }
}

impl<Ms> Feature<Ms> for Marker<Ms>
where
    Ms: 'static,
{
    fn into_node(&self) -> Node<Ms> {
        custom![
            Tag::from("leafy-marker"),
            attrs! {
                At::from("lat") => self.lat,
                At::from("lng") => self.lng,
                At::from("tooltip") => self.tooltip.as_ref().as_at_value(),
                At::from("tooltip-open") => self.tooltip_open.as_ref().as_at_value(),
            },
            self.on_click
                .clone()
                .map(|handler| ev(Ev::Click, move |_| handler())),
            self.on_mouseover
                .clone()
                .map(|handler| ev(Ev::MouseOver, move |_| handler())),
            self.on_mouseout
                .clone()
                .map(|handler| ev(Ev::MouseOut, move |_| handler())),
        ]
    }
}

// Circle

pub struct Circle<Ms> {
    lat: f64,
    lng: f64,
    radius: u32,
    stroke: Option<String>,
    stroke_width: Option<i32>,
    stroke_dasharray: Option<String>,
    fill: Option<String>,
    fill_opacity: Option<f64>,
    zoom_to_fit: Option<bool>,
    tooltip: Option<String>,
    tooltip_open: Option<bool>,
    on_click: Option<Rc<dyn Fn() -> Ms>>,
    on_mouseover: Option<Rc<dyn Fn() -> Ms>>,
    on_mouseout: Option<Rc<dyn Fn() -> Ms>>,
}

pub fn circle<Ms>(lat: f64, lng: f64, radius: u32) -> Circle<Ms> {
    Circle {
        lat,
        lng,
        radius,
        stroke: None,
        stroke_width: None,
        stroke_dasharray: None,
        fill: None,
        fill_opacity: None,
        zoom_to_fit: None,
        tooltip: None,
        tooltip_open: None,
        on_click: None,
        on_mouseover: None,
        on_mouseout: None,
    }
}

impl<Ms> Circle<Ms> {
    pub fn stroke(mut self, color: impl Into<String>) -> Self {
        self.stroke = Some(color.into());
        self
    }

    pub fn stroke_width(mut self, size: i32) -> Self {
        self.stroke_width = Some(size);
        self
    }

    pub fn stroke_dasharray<S: Into<String>>(mut self, dasharray: S) -> Self {
        self.stroke_dasharray = Some(dasharray.into());
        self
    }

    pub fn fill(mut self, color: impl Into<String>) -> Self {
        self.fill = Some(color.into());
        self
    }

    pub fn fill_opacity(mut self, opacity: f64) -> Self {
        self.fill_opacity = Some(opacity);
        self
    }

    pub fn zoom_to_fit(mut self) -> Self {
        self.zoom_to_fit = Some(true);
        self
    }

    pub fn tooltip<S: Into<String>>(mut self, value: S) -> Self {
        self.tooltip = Some(value.into());
        self
    }

    pub fn tooltip_open(mut self, value: bool) -> Self {
        self.tooltip_open = Some(value);
        self
    }

    pub fn on_click<F>(mut self, handler: F) -> Self
    where
        F: FnOnce() -> Ms + Clone + 'static,
    {
        self.on_click = Some(Rc::new(move || handler.clone()()));
        self
    }

    pub fn on_mouseover<F>(mut self, handler: F) -> Self
    where
        F: FnOnce() -> Ms + Clone + 'static,
    {
        self.on_mouseover = Some(Rc::new(move || handler.clone()()));
        self
    }

    pub fn on_mouseout<F>(mut self, handler: F) -> Self
    where
        F: FnOnce() -> Ms + Clone + 'static,
    {
        self.on_mouseout = Some(Rc::new(move || handler.clone()()));
        self
    }
}

impl<Ms> Feature<Ms> for Circle<Ms>
where
    Ms: 'static,
{
    fn into_node(&self) -> Node<Ms> {
        custom![
            Tag::from("leafy-circle"),
            attrs! {
                At::from("lat") => self.lat,
                At::from("lng") => self.lng,
                At::from("radius") => self.radius,
                At::from("stroke") => self.stroke.as_at_value(),
                At::from("stroke-width") => self.stroke_width.as_at_value(),
                At::from("stroke-dasharray") => self.stroke_dasharray.as_at_value(),
                At::from("fill") => self.fill.as_ref().as_at_value(),
                At::from("fill-opacity") => self.fill_opacity.as_at_value(),
                At::from("zoom-to-fit") => self.zoom_to_fit.as_at_value(),
                At::from("tooltip") => self.tooltip.as_ref().as_at_value(),
                At::from("tooltip-open") => self.tooltip_open.as_ref().as_at_value(),
            },
            self.on_click
                .clone()
                .map(|handler| ev(Ev::Click, move |_| handler())),
            self.on_mouseover
                .clone()
                .map(|handler| ev(Ev::MouseOver, move |_| handler())),
            self.on_mouseout
                .clone()
                .map(|handler| ev(Ev::MouseOut, move |_| handler())),
        ]
    }
}

// Geojson

pub struct Geojson<Ms> {
    data: String,
    key: Option<String>,
    stroke: Option<String>,
    stroke_width: Option<i32>,
    stroke_dasharray: Option<String>,
    fill: Option<String>,
    fill_opacity: Option<f64>,
    zoom_to_fit: Option<bool>,
    tooltip: Option<String>,
    tooltip_open: Option<bool>,
    on_click: Option<Rc<dyn Fn() -> Ms>>,
    on_mouseover: Option<Rc<dyn Fn() -> Ms>>,
    on_mouseout: Option<Rc<dyn Fn() -> Ms>>,
}

pub fn geojson<Ms, S: Into<String>>(data: S) -> Geojson<Ms> {
    Geojson {
        data: data.into(),
        key: None,
        stroke: None,
        stroke_width: None,
        stroke_dasharray: None,
        fill: None,
        fill_opacity: None,
        zoom_to_fit: None,
        tooltip: None,
        tooltip_open: None,
        on_click: None,
        on_mouseover: None,
        on_mouseout: None,
    }
}

impl<Ms> Geojson<Ms> {
    pub fn key(mut self, key: impl Into<String>) -> Self {
        self.key = Some(key.into());
        self
    }

    pub fn stroke(mut self, color: impl Into<String>) -> Self {
        self.stroke = Some(color.into());
        self
    }

    pub fn stroke_width(mut self, size: i32) -> Self {
        self.stroke_width = Some(size);
        self
    }

    pub fn stroke_dasharray<S: Into<String>>(mut self, dasharray: S) -> Self {
        self.stroke_dasharray = Some(dasharray.into());
        self
    }

    pub fn fill(mut self, color: impl Into<String>) -> Self {
        self.fill = Some(color.into());
        self
    }

    pub fn fill_opacity(mut self, opacity: f64) -> Self {
        self.fill_opacity = Some(opacity);
        self
    }

    pub fn zoom_to_fit(mut self) -> Self {
        self.zoom_to_fit = Some(true);
        self
    }

    pub fn tooltip<S: Into<String>>(mut self, value: S) -> Self {
        self.tooltip = Some(value.into());
        self
    }

    pub fn tooltip_open(mut self, value: bool) -> Self {
        self.tooltip_open = Some(value);
        self
    }

    pub fn on_click<F>(mut self, handler: F) -> Self
    where
        F: FnOnce() -> Ms + Clone + 'static,
    {
        self.on_click = Some(Rc::new(move || handler.clone()()));
        self
    }

    pub fn on_mouseover<F>(mut self, handler: F) -> Self
    where
        F: FnOnce() -> Ms + Clone + 'static,
    {
        self.on_mouseover = Some(Rc::new(move || handler.clone()()));
        self
    }

    pub fn on_mouseout<F>(mut self, handler: F) -> Self
    where
        F: FnOnce() -> Ms + Clone + 'static,
    {
        self.on_mouseout = Some(Rc::new(move || handler.clone()()));
        self
    }
}

impl<Ms> Feature<Ms> for Geojson<Ms>
where
    Ms: 'static,
{
    fn into_node(&self) -> Node<Ms> {
        custom![
            Tag::from("leafy-geojson"),
            self.key.as_ref().map(el_key),
            attrs! {
                At::from("data") => self.data,
                At::from("stroke") => self.stroke.as_at_value(),
                At::from("stroke-width") => self.stroke_width.as_at_value(),
                At::from("stroke-dasharray") => self.stroke_dasharray.as_at_value(),
                At::from("fill") => self.fill.as_ref().as_at_value(),
                At::from("fill-opacity") => self.fill_opacity.as_at_value(),
                At::from("zoom-to-fit") => self.zoom_to_fit.as_at_value(),
                At::from("tooltip") => self.tooltip.as_ref().as_at_value(),
                At::from("tooltip-open") => self.tooltip_open.as_ref().as_at_value(),
            },
            self.on_click
                .clone()
                .map(|handler| ev(Ev::Click, move |_| handler())),
            self.on_mouseover
                .clone()
                .map(|handler| ev(Ev::MouseOver, move |_| handler())),
            self.on_mouseout
                .clone()
                .map(|handler| ev(Ev::MouseOut, move |_| handler())),
        ]
    }
}
