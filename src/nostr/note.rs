use nostr_sdk::prelude::*;
// use indextree::Arena;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    KindNotMatch,
}

pub struct TextNote<'a> {
    inner_ref: &'a Event,
    root: Option<&'a EventId>,
    reply_to: Option<&'a EventId>,
}

impl<'a> TextNote<'a> {
    pub fn new(event: &'a Event) -> Self {
        TextNote {
            inner_ref: event,
            root: None,
            reply_to: None,
        }
    }

    pub fn is_root(&self) -> bool {
        self.root.is_none() && self.reply_to.is_none()
    }
}

impl<'a> TryFrom<&'a Event> for TextNote<'a> {
    type Error = Error;

    fn try_from(event: &'a Event) -> Result<Self, Self::Error> {
        if event.kind == Kind::TextNote {
            let mut text_note = TextNote::new(event);
            let mut no_marker_array: Vec<&EventId> = vec![];
            //try to use marker to match root & reply_to
            event.iter_tags().for_each(|t| {
                if let Tag::Event {
                    event_id,
                    relay_url: _,
                    marker,
                } = t
                {
                    match marker {
                        Some(Marker::Root) => text_note.root = Some(event_id),
                        Some(Marker::Reply) => text_note.reply_to = Some(event_id),
                        None => no_marker_array.push(event_id),
                        _ => {} //do nothing
                    }
                }
            });
            // a reply for root
            if text_note.root.is_none() && text_note.reply_to.is_some() {
                text_note.root = text_note.reply_to;
            }
            // no marker at all
            if text_note.reply_to.is_none() {
                match no_marker_array.len() {
                    1 => {
                        text_note.root = no_marker_array.first().copied();
                        text_note.reply_to = no_marker_array.first().copied();
                    }
                    n if n >= 2 => {
                        text_note.root = no_marker_array.first().copied();
                        text_note.reply_to = no_marker_array.get(1).copied();
                    }
                    _ => {}
                }
            }
            Ok(text_note)
        } else {
            Err(Error::KindNotMatch)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_note() {
        let event = Event::from_json(r#"{"pubkey":"e1ff3bfdd4e40315959b08b4fcc8245eaa514637e1d4ec2ae166b743341be1af","sig":"53087c94115efb071632abc8d514b1f09b20eb8377d2854fa51ab76c4ac0aa6c5766c9af45ded4e2789098bad73117e02d0bdcb96c34866bec1898753a80465a","created_at":1713418044,"id":"0076792624df92e4b0892722c282fdeddd5912e89d61af843e180f2dc02a5530","content":"Pornalreadyfillsthedemandforaigirlfriends.Theaipartonlyaddssomeonetotalkto,whichboysdon'treallycrave.Theaiboyfriendwillbeabiggerdeal,womengetsomeonetotalktoalldaywithoutanythingelse.","kind":6,"tags":[["e","1c556c3a9e892841bef2bfae13ca5fdc50f81054d031a6a16b060a2e5113ae24"],["p","0018b7ee33fb253843639c62e292fec700a69a93b08ee374c5bda971c9b39564"]]}"#).unwrap();
        assert!(TextNote::try_from(&event).is_err(), "Expect an event with kind 1");
    }

    #[test]
    fn test_reply_with_marker() {
        let event = Event::from_json(r#"{"id":"e36817d0509cdd99d854391027bef6f3a0af1d87bdbdb1d9eb73201ff1719e09","kind":1,"pubkey":"77953b3a63bcf1c748dbdeef109bd56de48c30edcd27d2092440c3adca31c975","tags":[["e","39413ed0400101a45abb82dd8949306790234f785ea224717d0f68fa1b36e935","","root"],["e","3cacfcc0afa9d1daf798291b8d8b31fd0b471303f501e188191444ff4cdf1345","","reply"],["p","58ead82fa15b550094f7f5fe4804e0fe75b779dbef2e9b20511eccd69e6d08f9"],["p","460c25e682fda7832b52d1f22d3d22b3176d972f60dcdc3212ed8c92ef85065c"],["p","6e468422dfb74a5738702a8823b9b28168abab8655faacb6853cd0ee15deee93"],["p","77953b3a63bcf1c748dbdeef109bd56de48c30edcd27d2092440c3adca31c975"]],"created_at":1713443749,"content":"Isee.Thanks!","sig":"7b6f820665a7e52b6b655985fbe1287cbd57304b06af68f9d410d0c89e60a69b9c71698fccca7ebeb192d3d004bdb2e1f3eb1fe5352c68a0021cd8d56c1a4218"}"#).unwrap();
        let textNote = TextNote::try_from(&event).unwrap();
        assert!(textNote.root.unwrap().to_hex() == *"39413ed0400101a45abb82dd8949306790234f785ea224717d0f68fa1b36e935");
        assert!(textNote.reply_to.unwrap().to_hex() == *"3cacfcc0afa9d1daf798291b8d8b31fd0b471303f501e188191444ff4cdf1345");
    }

    #[test]
    fn test_reply_with_no_marker() {
        let event = Event::from_json(r#"{"content":"Wow how did i only just get this 🤦‍♂️","created_at":1713415596,"id":"0646ee437c5fc88d90a8c9b846edce3611e8a6e8545e952dbd7975f4a52925bb","kind":1,"pubkey":"32e1827635450ebb3c5a7d12c1f8e7b2b514439ac10a67eef3d9fd9c5c68e245","sig":"bff8feafd44078c69402d8d7b3cd5148489d86b8a36ccf28c704920c776b1e568d6556743079866a5d33d70900f3c6fa09e3b0e02bf1f6d7a6a2394873623243","tags":[["e","a200b725177cc2fcbb0c40c5103695da6a8cbd9e73c5a9293c8bfd45521a84bc"],["e","cfab5dabf95fa14c21a611a3eff120132a470201407bd6799ae1c5058b88b430"],["p","79c2cae114ea28a981e7559b4fe7854a473521a8d22a66bbab9fa248eb820ff6"],["p","07457649b3ae6b2c21bee53c5012a2c1f4f6353bb360feebf52959bab4486d67"],["p","deba271e547767bd6d8eec75eece5615db317a03b07f459134b03e7236005655"]]}"#).unwrap();
        let textNote = TextNote::try_from(&event).unwrap();
        assert!(textNote.root.unwrap().to_hex() == *"a200b725177cc2fcbb0c40c5103695da6a8cbd9e73c5a9293c8bfd45521a84bc");
        assert!(textNote.reply_to.unwrap().to_hex() == *"cfab5dabf95fa14c21a611a3eff120132a470201407bd6799ae1c5058b88b430");
    }

    #[test]
    fn test_reply_to_root_no_marker() {
        let event = Event::from_json(r#"{"content":"Porn already fills the demand for ai girlfriends. The ai part only adds someone to talk to, which boys don't really crave. The ai boyfriend will be a bigger deal, women get someone to talk to all day without anything else.","created_at":1713418044,"id":"0076792624df92e4b0892722c282fdeddd5912e89d61af843e180f2dc02a5530","kind":1,"pubkey":"e1ff3bfdd4e40315959b08b4fcc8245eaa514637e1d4ec2ae166b743341be1af","sig":"53087c94115efb071632abc8d514b1f09b20eb8377d2854fa51ab76c4ac0aa6c5766c9af45ded4e2789098bad73117e02d0bdcb96c34866bec1898753a80465a","tags":[["e","1c556c3a9e892841bef2bfae13ca5fdc50f81054d031a6a16b060a2e5113ae24"],["p","0018b7ee33fb253843639c62e292fec700a69a93b08ee374c5bda971c9b39564"]]}"#).unwrap();
        let textNote = TextNote::try_from(&event).unwrap();
        assert!(textNote.root.unwrap().to_hex() == *"1c556c3a9e892841bef2bfae13ca5fdc50f81054d031a6a16b060a2e5113ae24");
        assert!(textNote.reply_to.unwrap().to_hex() == *"1c556c3a9e892841bef2bfae13ca5fdc50f81054d031a6a16b060a2e5113ae24");
    }
}
