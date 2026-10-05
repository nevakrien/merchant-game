use std::sync::Arc;

pub type Count = u64;
pub type Tick = u64;

pub const NUM_ITEMS : usize = 2;

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Item {
    Money,
    Potato,
}

macro_rules! id_type {
    ($name:ident) => {
        #[derive(
            Debug,
            Clone,
            Copy,
            PartialEq,
            Eq,
            PartialOrd,
            Ord,
            Hash,
        )]
        pub struct $name(pub u32);

        impl $name {
            pub const fn new(value: u32) -> Self {
                Self(value)
            }

            pub const fn index(self) -> usize {
                self.0 as usize
            }
        }
    };
}

id_type!(CityId);
id_type!(MerchantId);
id_type!(RoadId);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ItemStack {
    pub item: Item,
    pub count: Count,
}

#[derive(Debug, Clone)]
pub struct Order {
    /// Maximum number of times this order may be executed.
    ///
    /// For example, an order:
    ///
    ///     take:  5 Money
    ///     give:  4 Potato
    ///     max_times: 10
    ///
    /// may be partially filled from 1..=10 times.
    pub max_times: Count,

    pub taken: Vec<ItemStack>,
    pub given: Vec<ItemStack>,
}

#[derive(Debug, Clone)]
pub struct MemoryEvent {
    pub buyer:MerchantId,
    pub seller:MerchantId,
    pub city:CityId,
    pub order:Order,
    pub filled:Count,
    pub tick:Tick
}

pub type Inventory = [Count;NUM_ITEMS];

pub enum Action {
    Barter,
    MovePartners(MerchantId),
    MoveCities
}

pub trait Policy {
    fn buy_offer(&mut self,mine:&Inventory,order:&Order)->Count;
    fn make_offer(&mut self,mine:&Inventory)->Option<Order>;
    fn update_memory(&mut self,event:&MemoryEvent);

    fn choose_next_partner(&mut self,mine:&Inventory,merchants:&[MerchantId],current:MerchantId)->Action;
    fn choose_next_city(&mut self,mine:&Inventory,graph:&[Vec<(CityId,Tick)>],current:CityId)->CityId;
}

pub struct Merchant{
    order:Option<Order>,
    items:Inventory,

    id:MerchantId,

    pub policy:Box<dyn Policy>,
    pub rumors:Vec<Arc<MemoryEvent>>,

    //we might want an RNG seed for each merchant sepratly?
    //this makes some parallalisem stuff easier to do determinstically
}

impl Merchant {
    #[inline(always)]
    pub fn gossip(&self)->Option<&Arc<MemoryEvent>>{
        todo!("choose a random memory event to gossip about")
    }

    #[inline(always)]
    pub fn add_gossip(&mut self,gossip:&Arc<MemoryEvent>){
        //maybe forget here to keep size consistent? or just ring buffer
        self.rumors.push(gossip.clone());
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GamePhase {
    Travel {
        road: RoadId,
        arrive_tick: Tick,
    },

    City(CityId),

    Trade(MerchantId),
}

pub fn check_buy_order(buyer:&mut Merchant,seller:&mut Merchant,city:CityId,tick:Tick){
    if let Some(o) = &mut seller.order {
        let filled = buyer.policy.buy_offer(&buyer.items,&o);
        if filled==0 { return };

        assert!(filled<=o.max_times);
        for t in &o.taken {
            assert!(seller.items[t.item as usize]>=t.count*filled);
        }
        for t in &o.given {
            assert!(buyer.items[t.item as usize]>=t.count*filled);
        }

        let order = o.clone();
        o.max_times -= filled;
        
        for t in &o.taken {
           seller.items[t.item as usize]-=t.count*filled;
           buyer.items[t.item as usize]+=t.count*filled;
        }
        for t in &o.given {
            buyer.items[t.item as usize]-=t.count*filled;
            seller.items[t.item as usize]+=t.count*filled;
        }

        //TODO generate gossip about this at some probabilety for both parties
        let event = Arc::new(MemoryEvent {
            buyer:buyer.id,
            seller:seller.id,
            filled,
            order,
            city,
            tick

        });

        buyer.add_gossip(&event);
        seller.add_gossip(&event);

    }
}

pub fn handle_barter_round(buyer:&mut Merchant,seller:&mut Merchant,city:CityId,tick:Tick){
    check_buy_order(buyer,seller,city,tick);
    seller.order = seller.policy.make_offer(&seller.items);

    check_buy_order(seller,buyer,city,tick);
    buyer.order = buyer.policy.make_offer(&buyer.items);
}

pub fn engage_gossip(teller:&Merchant,listener:&mut Merchant){
    if let Some(event) = teller.gossip() {
        listener.policy.update_memory(event);
        //maybe forget here to keep size consistent?
        listener.add_gossip(event)
    }
}

pub fn maybe_move_merchant(m:&mut Merchant,merchants:&[MerchantId],current:MerchantId)->Action{
     m.policy.choose_next_partner(&m.items,merchants,current)
}

pub fn maybe_move_cities(m:&mut Merchant,graph:&[Vec<(CityId,Tick)>],city:CityId)->Option<CityId>{
    let next = m.policy.choose_next_city(&m.items,graph,city);
    if next != city {
        Some(next)
    }else {
        None
    }
}

pub struct Game {
    pub merchants:Vec<Merchant>,
    pub cities: Vec<Vec<MerchantId>>,
    pub graph:Vec<Vec<(CityId,Tick)>>,
}

fn main() {
    println!("Hello, world!");
}