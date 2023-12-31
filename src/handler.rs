use crate::context::Context;

/// ```
/// #[async_trait::async_trait]
/// pub trait Handler<T>{
///   async fn active(&mut self,ctx:&Context<T>);
///   async fn read(&mut self,ctx:&Context<T>,msg:T);
///   async fn close(&mut self,ctx:&Context<T>);
/// }
/// 
/// pub struct MsgHandler {}
/// 
/// #[async_trait::async_trait]
/// impl Handler<Msg> for MsgHandler {
///     
///     async fn read(&mut self, ctx: &Context<Msg>, msg: Msg) {
///         println!("read {:?}", msg);
///         let m=Msg::new(TypesEnum::ProxyOpen, msg.data);
///         ctx.write(m).await;
///     }
///     async fn active(&mut self, ctx: &Context<Msg>) {
///         
///         println!("active {:?}", ctx.addr());
///         
///     }
///     async fn close(&mut self,ctx:&Context<Msg>){
///         println!("close {:?} ", ctx.addr());
///     }
/// }
/// 
/// ```
/// 
#[async_trait::async_trait]
pub trait Handler<T>{
  async fn active(&mut self,ctx:&Context<T>);
  async fn read(&mut self,ctx:&Context<T>,msg:T);
  async fn close(&mut self,ctx:&Context<T>);
}