pub mod element;

use self::element::Element;
use num::traits::Zero;
use ndarray::prelude::*;
use ndarray::iter::{Iter, IterMut};
use ndarray::NdIndex;

/// Storage for a patterned collection of image elements.
pub struct ImageStorage<E: Element, D: Dimension> {
    data: Array<E, D>,
}

impl<E: Element, D: Dimension> ImageStorage<E, D> {
    pub fn from_elem<S>(shape: S, elem: E) -> Self
    where
        E: Clone,
        S: ShapeBuilder<Dim = D>,
    {
        ImageStorage {
            data: Array::from_elem(shape, elem),
        }
    }

    pub fn empty<S>(shape: S) -> Self
    where
        E: Clone + Zero,
        S: ShapeBuilder<Dim = D>,
    {
        ImageStorage {
            data: Array::zeros(shape),
        }
    }

    pub fn default<S>(shape: S) -> Self
    where
        E: Default,
        S: ShapeBuilder<Dim = D>,
    {
        ImageStorage {
            data: Array::default(shape),
        }
    }

    pub fn elements(&self) -> Elements<E, D> {
        Elements {
            buffer: self.data.iter(),
        }
    }

    pub fn elements_mut(&mut self) -> ElementsMut<E, D> {
        ElementsMut {
            buffer: self.data.iter_mut(),
        }
    }

    pub fn dimensions(&self) -> &[usize] {
        self.data.shape()
    }

    pub fn get_element<I>(&self, index: I) -> Option<&E> 
    where 
        I: NdIndex<D>,
    {
        self.data.get(index)
    }

    pub fn get_element_mut<I>(&mut self, index: I) -> Option<&mut E> 
    where
        I: NdIndex<D>,
    {
        self.data.get_mut(index)
    }
}

/// Specialization of image storage for two-dimensional images.
pub type Image2D<E> = ImageStorage<E, Ix2>;

/// Specialization of image storage for three-dimensional image volumes.
pub type Image3D<E> = ImageStorage<E, Ix3>;

/// Iterator structure that provides element-wise access to the underlying element buffer.
pub struct Elements<'a, E: 'a, D> {
    buffer: Iter<'a, E, D>,
}

impl<'a, E, D> Iterator for Elements<'a, E, D>
where
    E: Element,
    D: Dimension,
{
    type Item = &'a E;

    fn next(&mut self) -> Option<Self::Item> {
        self.buffer.next()
    }
}

/// Iterator structure that provides element-wise mutable access to the underlying element buffer.
pub struct ElementsMut<'a, E: 'a, D> {
    buffer: IterMut<'a, E, D>,
}

impl<'a, E, D> Iterator for ElementsMut<'a, E, D>
where
    E: Element,
    D: Dimension,
{
    type Item = &'a mut E;

    fn next(&mut self) -> Option<Self::Item> {
        self.buffer.next()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn image_storage_from_elem_test() {
        let mut image =
            ImageStorage::from_elem((10, 10, 10), element::RGB::new(0.0f32, 1.0f32, 0.5f32));
    }
}
