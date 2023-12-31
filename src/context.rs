use std::any::Any;
use std::collections::HashMap;
use std::fmt::Debug;
use std::net::SocketAddr;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::sync::{Mutex, RwLock};
use tokio::sync::mpsc::{Sender};

pub struct Context<T> {
    in_tx: Arc<Mutex<Sender<T>>>,
    out_tx: Arc<Mutex<Sender<T>>>,
    close_tx:Arc<Mutex<Sender<()>>>,
    attributes:Arc<RwLock<HashMap<String, Arc<Mutex<Box<dyn Any + Send + Sync>>>>>>,
    addr:SocketAddr,
    is_run:Arc<AtomicBool>,
}

impl<T> Debug for Context<T>{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Context").field(&self.addr).finish()
    }
}

impl<T> PartialEq for Context<T>{
    fn eq(&self, other: &Self) -> bool {
        self.addr == other.addr
    }
}


impl<T> Clone for Context<T> {
    fn clone(&self) -> Self {
        Context {
            in_tx:Arc::clone(&self.in_tx),
            out_tx:Arc::clone(&self.out_tx),
            close_tx:Arc::clone(&self.close_tx),
            attributes:Arc::clone(&self.attributes),
            addr:self.addr.clone(),
            is_run:Arc::clone(&self.is_run),
        }
    }

    fn clone_from(&mut self, source: &Self) {
        *self = source.clone()
    }
}

impl<T> Context<T> {
    pub fn new(in_tx: Sender<T>, out_tx: Sender<T>,addr:SocketAddr,close_tx:Sender<()>) -> Self {
        Context {
            in_tx: Arc::new(Mutex::new(in_tx)),
            out_tx: Arc::new(Mutex::new(out_tx)),
            close_tx:Arc::new(Mutex::new(close_tx)),
            attributes:Arc::new(RwLock::new(HashMap::new())),
            addr,
            is_run:Arc::new(AtomicBool::new(true))
        }
    }

    

    #[allow(dead_code)] 
    pub fn addr(&self)->SocketAddr{
      self.addr
    }

    pub async fn out(&self, msg: T)->Result<(),ContextError>
    where
        T: Send + 'static,
    {
        let s=self.out_tx.lock().await;
        
        match s.send(msg).await {
            Ok(_) => {
                Ok(())
            },
            Err(e)=>{
                Err(ContextError::new(e.to_string()))
            }
        }
    }
    pub async fn write(&self, msg:T) ->Result<(),ContextError>{
        match self.in_tx.try_lock() {
            Ok(in_tx) => {
                match in_tx.send(msg).await {
                    Ok(_) => {
                        Ok(())
                    },
                    Err(e)=>{
                        Err(ContextError::new(e.to_string()))
                    }
                }   
            },
            Err(e) => {
                Err(ContextError::new(format!("{:?}",e)))
            },
        }
        // self.in_tx.lock().await.send(msg).await;
    }

    pub async fn close(&self)->Result<(),ContextError> {
        self.is_run.store(false, Ordering::Relaxed);
        match self.close_tx.lock().await.send(()).await {
            Ok(_) => Ok(()),
            Err(e) => Err(ContextError::new(e.to_string()))
        }
    }

    pub fn is_run(&self)->bool{
        self.is_run.load(Ordering::Relaxed)
    }

    pub async fn set_attribute(&self, key: String, value: Box<dyn Any + Send + Sync>) {
        // let mut attributes = self.attributes.lock().await;
        self.attributes.write().await.insert(key, Arc::new(Mutex::new(value)));
        // attributes.insert(key, Arc::new(Mutex::new(value)));
    }

    pub async fn remove_attribute(&self, key: String) {
        // let mut attributes = self.attributes.lock().await;
        // attributes.remove(&key);
        self.attributes.write().await.remove(&key);
    }

    pub async fn get_attribute(&self, key: String) -> Arc<Mutex<Box<dyn Any + Send + Sync>>> {
        // let attributes = self.attributes.lock().await;
        // let v = attributes.get(key.as_str());
        match self.attributes.read().await.get(key.as_str()){
            Some(v)=>Arc::clone(v),
            None=>Arc::new(Mutex::new(Box::new(())))
        }
    }

}



use std::error::Error;

#[derive(Debug)]
pub struct ContextError {
    details: String
}

impl ContextError {
    fn new(msg: String) -> ContextError {
        ContextError{details: msg}
    }
}

impl std::fmt::Display for ContextError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for ContextError {
    fn description(&self) -> &str {
        &self.details
    }

    fn cause(&self) -> Option<&dyn Error> {
        None 
    }

    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}
