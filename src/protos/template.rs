// Automatically generated rust module for 'db.proto' file

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
#![allow(unknown_lints)]
#![allow(clippy::all)]
#![cfg_attr(rustfmt, rustfmt_skip)]

use std::borrow::Cow;
use std::collections::BTreeMap;

use quick_protobuf::{BytesReader, MessageRead, MessageWrite, Result, Writer, WriterBackend};
use quick_protobuf::sizeofs::*;

use super::*;

pub(crate) type KVMap<K, V> = BTreeMap<K, V>;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Language {
    English = 0,
    Chinese = 1,
    Japanese = 2,
    German = 3,
    French = 4,
}

impl Default for Language {
    fn default() -> Self {
        Language::English
    }
}

impl From<i32> for Language {
    fn from(i: i32) -> Self {
        match i {
            0 => Language::English,
            1 => Language::Chinese,
            2 => Language::Japanese,
            3 => Language::German,
            4 => Language::French,
            _ => Self::default(),
        }
    }
}


impl<'a> From<&'a str> for Language {
    fn from(s: &'a str) -> Self {
        match s {
            "English" => Language::English,
            "Chinese" => Language::Chinese,
            "Japanese" => Language::Japanese,
            "German" => Language::German,
            "French" => Language::French,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct DB<'a> {
    pub Items: Vec<Cow<'a, [u8]>>,
    pub JobCategories: Vec<Cow<'a, [u8]>>,
    pub Masterpieces: Vec<Cow<'a, [u8]>>,
    pub CraftLevelDifferences: Vec<Cow<'a, [u8]>>,
    pub Currencies: Vec<Cow<'a, [u8]>>,
    pub Books: Vec<Cow<'a, [u8]>>,
    pub Areas: Vec<Cow<'a, [u8]>>,
    pub Aetherytes: Vec<Cow<'a, [u8]>>,
    pub RouterPaths: Vec<Cow<'a, [u8]>>,
    pub GatherSpots: Vec<Cow<'a, [u8]>>,
    pub EphemeralSpots: Vec<Cow<'a, [u8]>>,
    pub UnspoiledSpots: Vec<Cow<'a, [u8]>>,
    pub GrindSpots: Vec<Cow<'a, [u8]>>,
    pub Recipes: Vec<Cow<'a, [u8]>>,
    pub Npcs: Vec<Cow<'a, [u8]>>,
    pub FishingEntries: Vec<Cow<'a, [u8]>>,
    pub Shops: Vec<Cow<'a, [u8]>>,
    pub Reputations: Vec<Cow<'a, [u8]>>,
    pub GatherSkills: Vec<Cow<'a, [u8]>>,
    pub FishingSpots: Vec<Cow<'a, [u8]>>,
    pub Leves: Vec<Cow<'a, [u8]>>,
    pub TranslationMaps: Vec<template::TranslationMaps<'a>>,
    pub Weather: Vec<Cow<'a, [u8]>>,
    pub LevelingBlocks: Vec<Cow<'a, [u8]>>,
    pub Mounts: Vec<Cow<'a, [u8]>>,
    pub FishingRecords: Vec<Cow<'a, [u8]>>,
    pub DiademSearchpointCircuit: Vec<Cow<'a, [u8]>>,
    pub ResourceApprovals: Vec<Cow<'a, [u8]>>,
    pub SummoningBellIds: Cow<'a, [u8]>,
    pub RetainerNames: Vec<Cow<'a, [u8]>>,
    pub DiademSpots: Vec<Cow<'a, [u8]>>,
    pub Quests: Vec<Cow<'a, [u8]>>,
    pub CircuitSpots: Vec<Cow<'a, [u8]>>,
    pub SextantZoneHeaders: Vec<Cow<'a, [u8]>>,
    pub Landpoints: Vec<Cow<'a, [u8]>>,
    pub Materia: Vec<Cow<'a, [u8]>>,
    pub Consumables: Vec<Cow<'a, [u8]>>,
    pub ItemSeries: Vec<Cow<'a, [u8]>>,
    pub DiademMobs: Vec<Cow<'a, [u8]>>,
    pub TreasureMaps: Cow<'a, [u8]>,
    pub IshgardPoints: Cow<'a, [u8]>,
}

impl<'a> MessageRead<'a> for DB<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.Items.push(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(18) => msg.JobCategories.push(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(26) => msg.Masterpieces.push(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(34) => msg.CraftLevelDifferences.push(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(42) => msg.Currencies.push(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(50) => msg.Books.push(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(58) => msg.Areas.push(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(66) => msg.Aetherytes.push(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(74) => msg.RouterPaths.push(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(82) => msg.GatherSpots.push(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(90) => msg.EphemeralSpots.push(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(98) => msg.UnspoiledSpots.push(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(106) => msg.GrindSpots.push(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(114) => msg.Recipes.push(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(122) => msg.Npcs.push(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(130) => msg.FishingEntries.push(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(138) => msg.Shops.push(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(146) => msg.Reputations.push(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(154) => msg.GatherSkills.push(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(162) => msg.FishingSpots.push(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(170) => msg.Leves.push(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(178) => msg.TranslationMaps.push(r.read_message::<template::TranslationMaps>(bytes)?),
                Ok(186) => msg.Weather.push(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(194) => msg.LevelingBlocks.push(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(202) => msg.Mounts.push(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(210) => msg.FishingRecords.push(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(218) => msg.DiademSearchpointCircuit.push(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(226) => msg.ResourceApprovals.push(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(234) => msg.SummoningBellIds = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(242) => msg.RetainerNames.push(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(250) => msg.DiademSpots.push(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(258) => msg.Quests.push(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(266) => msg.CircuitSpots.push(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(274) => msg.SextantZoneHeaders.push(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(282) => msg.Landpoints.push(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(290) => msg.Materia.push(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(298) => msg.Consumables.push(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(306) => msg.ItemSeries.push(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(314) => msg.DiademMobs.push(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(322) => msg.TreasureMaps = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(330) => msg.IshgardPoints = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for DB<'a> {
    fn get_size(&self) -> usize {
        0
            + self.Items.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
            + self.JobCategories.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
            + self.Masterpieces.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
            + self.CraftLevelDifferences.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
            + self.Currencies.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
            + self.Books.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
            + self.Areas.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
            + self.Aetherytes.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
            + self.RouterPaths.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
            + self.GatherSpots.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
            + self.EphemeralSpots.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
            + self.UnspoiledSpots.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
            + self.GrindSpots.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
            + self.Recipes.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
            + self.Npcs.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
            + self.FishingEntries.iter().map(|s| 2 + sizeof_len((s).len())).sum::<usize>()
            + self.Shops.iter().map(|s| 2 + sizeof_len((s).len())).sum::<usize>()
            + self.Reputations.iter().map(|s| 2 + sizeof_len((s).len())).sum::<usize>()
            + self.GatherSkills.iter().map(|s| 2 + sizeof_len((s).len())).sum::<usize>()
            + self.FishingSpots.iter().map(|s| 2 + sizeof_len((s).len())).sum::<usize>()
            + self.Leves.iter().map(|s| 2 + sizeof_len((s).len())).sum::<usize>()
            + self.TranslationMaps.iter().map(|s| 2 + sizeof_len((s).get_size())).sum::<usize>()
            + self.Weather.iter().map(|s| 2 + sizeof_len((s).len())).sum::<usize>()
            + self.LevelingBlocks.iter().map(|s| 2 + sizeof_len((s).len())).sum::<usize>()
            + self.Mounts.iter().map(|s| 2 + sizeof_len((s).len())).sum::<usize>()
            + self.FishingRecords.iter().map(|s| 2 + sizeof_len((s).len())).sum::<usize>()
            + self.DiademSearchpointCircuit.iter().map(|s| 2 + sizeof_len((s).len())).sum::<usize>()
            + self.ResourceApprovals.iter().map(|s| 2 + sizeof_len((s).len())).sum::<usize>()
            + if self.SummoningBellIds == Cow::Borrowed(b"") { 0 } else { 2 + sizeof_len((&self.SummoningBellIds).len()) }
            + self.RetainerNames.iter().map(|s| 2 + sizeof_len((s).len())).sum::<usize>()
            + self.DiademSpots.iter().map(|s| 2 + sizeof_len((s).len())).sum::<usize>()
            + self.Quests.iter().map(|s| 2 + sizeof_len((s).len())).sum::<usize>()
            + self.CircuitSpots.iter().map(|s| 2 + sizeof_len((s).len())).sum::<usize>()
            + self.SextantZoneHeaders.iter().map(|s| 2 + sizeof_len((s).len())).sum::<usize>()
            + self.Landpoints.iter().map(|s| 2 + sizeof_len((s).len())).sum::<usize>()
            + self.Materia.iter().map(|s| 2 + sizeof_len((s).len())).sum::<usize>()
            + self.Consumables.iter().map(|s| 2 + sizeof_len((s).len())).sum::<usize>()
            + self.ItemSeries.iter().map(|s| 2 + sizeof_len((s).len())).sum::<usize>()
            + self.DiademMobs.iter().map(|s| 2 + sizeof_len((s).len())).sum::<usize>()
            + if self.TreasureMaps == Cow::Borrowed(b"") { 0 } else { 2 + sizeof_len((&self.TreasureMaps).len()) }
            + if self.IshgardPoints == Cow::Borrowed(b"") { 0 } else { 2 + sizeof_len((&self.IshgardPoints).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.Items { w.write_with_tag(10, |w| w.write_bytes(&**s))?; }
        for s in &self.JobCategories { w.write_with_tag(18, |w| w.write_bytes(&**s))?; }
        for s in &self.Masterpieces { w.write_with_tag(26, |w| w.write_bytes(&**s))?; }
        for s in &self.CraftLevelDifferences { w.write_with_tag(34, |w| w.write_bytes(&**s))?; }
        for s in &self.Currencies { w.write_with_tag(42, |w| w.write_bytes(&**s))?; }
        for s in &self.Books { w.write_with_tag(50, |w| w.write_bytes(&**s))?; }
        for s in &self.Areas { w.write_with_tag(58, |w| w.write_bytes(&**s))?; }
        for s in &self.Aetherytes { w.write_with_tag(66, |w| w.write_bytes(&**s))?; }
        for s in &self.RouterPaths { w.write_with_tag(74, |w| w.write_bytes(&**s))?; }
        for s in &self.GatherSpots { w.write_with_tag(82, |w| w.write_bytes(&**s))?; }
        for s in &self.EphemeralSpots { w.write_with_tag(90, |w| w.write_bytes(&**s))?; }
        for s in &self.UnspoiledSpots { w.write_with_tag(98, |w| w.write_bytes(&**s))?; }
        for s in &self.GrindSpots { w.write_with_tag(106, |w| w.write_bytes(&**s))?; }
        for s in &self.Recipes { w.write_with_tag(114, |w| w.write_bytes(&**s))?; }
        for s in &self.Npcs { w.write_with_tag(122, |w| w.write_bytes(&**s))?; }
        for s in &self.FishingEntries { w.write_with_tag(130, |w| w.write_bytes(&**s))?; }
        for s in &self.Shops { w.write_with_tag(138, |w| w.write_bytes(&**s))?; }
        for s in &self.Reputations { w.write_with_tag(146, |w| w.write_bytes(&**s))?; }
        for s in &self.GatherSkills { w.write_with_tag(154, |w| w.write_bytes(&**s))?; }
        for s in &self.FishingSpots { w.write_with_tag(162, |w| w.write_bytes(&**s))?; }
        for s in &self.Leves { w.write_with_tag(170, |w| w.write_bytes(&**s))?; }
        for s in &self.TranslationMaps { w.write_with_tag(178, |w| w.write_message(s))?; }
        for s in &self.Weather { w.write_with_tag(186, |w| w.write_bytes(&**s))?; }
        for s in &self.LevelingBlocks { w.write_with_tag(194, |w| w.write_bytes(&**s))?; }
        for s in &self.Mounts { w.write_with_tag(202, |w| w.write_bytes(&**s))?; }
        for s in &self.FishingRecords { w.write_with_tag(210, |w| w.write_bytes(&**s))?; }
        for s in &self.DiademSearchpointCircuit { w.write_with_tag(218, |w| w.write_bytes(&**s))?; }
        for s in &self.ResourceApprovals { w.write_with_tag(226, |w| w.write_bytes(&**s))?; }
        if self.SummoningBellIds != Cow::Borrowed(b"") { w.write_with_tag(234, |w| w.write_bytes(&**&self.SummoningBellIds))?; }
        for s in &self.RetainerNames { w.write_with_tag(242, |w| w.write_bytes(&**s))?; }
        for s in &self.DiademSpots { w.write_with_tag(250, |w| w.write_bytes(&**s))?; }
        for s in &self.Quests { w.write_with_tag(258, |w| w.write_bytes(&**s))?; }
        for s in &self.CircuitSpots { w.write_with_tag(266, |w| w.write_bytes(&**s))?; }
        for s in &self.SextantZoneHeaders { w.write_with_tag(274, |w| w.write_bytes(&**s))?; }
        for s in &self.Landpoints { w.write_with_tag(282, |w| w.write_bytes(&**s))?; }
        for s in &self.Materia { w.write_with_tag(290, |w| w.write_bytes(&**s))?; }
        for s in &self.Consumables { w.write_with_tag(298, |w| w.write_bytes(&**s))?; }
        for s in &self.ItemSeries { w.write_with_tag(306, |w| w.write_bytes(&**s))?; }
        for s in &self.DiademMobs { w.write_with_tag(314, |w| w.write_bytes(&**s))?; }
        if self.TreasureMaps != Cow::Borrowed(b"") { w.write_with_tag(322, |w| w.write_bytes(&**&self.TreasureMaps))?; }
        if self.IshgardPoints != Cow::Borrowed(b"") { w.write_with_tag(330, |w| w.write_bytes(&**&self.IshgardPoints))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct TranslationMaps<'a> {
    pub Language: template::Language,
    pub KeyToName: Vec<KVMap<Cow<'a,str>,Cow<'a,str>>>,
}


impl<'a> MessageRead<'a> for TranslationMaps<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.Language = r.read_enum(bytes)?,
                Ok(18) => {
                    let (key, value) = r.read_map(bytes, |r, bytes| Ok(r.read_string(bytes).map(Cow::Borrowed)?), |r, bytes| Ok(r.read_string(bytes).map(Cow::Borrowed)?))?;
                    let mut fg = KVMap::new();
                    fg.insert(key,value);
                    msg.KeyToName.push(fg)//.insert(key , value);
                }
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for TranslationMaps<'a> {
    fn get_size(&self) -> usize {
        0
            + if self.Language == template::Language::English { 0 } else { 1 + sizeof_varint(*(&self.Language) as u64) }
            + self.KeyToName.iter().flatten().map(|(k, v)| 1 + sizeof_len(2 + sizeof_len((k).len()) + sizeof_len((v).len()))).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.Language != template::Language::English {
            w.write_with_tag(8, |w| w.write_enum(*&self.Language as i32))?;
        }
        for (k, v) in self.KeyToName.iter().flatten() {
            w.write_with_tag(18, |w| w.write_map(2 + sizeof_len((k).len()) + sizeof_len((v).len()), 10, |w| w.write_string(&**k), 18, |w| w.write_string(&**v)))?;
        }
        Ok(())
    }
}

